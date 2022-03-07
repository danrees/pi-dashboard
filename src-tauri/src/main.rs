#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod calendar;
mod errors;
mod google;

use errors::DashboardError;
use google::auth;
use google::client::{CalendarList, Client, EventList};

use calendar::Config as CalendarConfig;
use calendar::State;
use config::Config;
use rocket;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use tauri::{api::shell, Manager, Window};

struct Tx(Mutex<Sender<String>>);
struct Rx(Mutex<Receiver<String>>);

#[derive(Serialize, Deserialize, Debug, Clone)]
enum WeatherUnits {
  #[serde(rename = "metric")]
  Metric,
  #[serde(rename = "imperial")]
  Imperial,
}

impl std::fmt::Display for WeatherUnits {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      WeatherUnits::Metric => write!(f, "metric"),
      WeatherUnits::Imperial => write!(f, "imperial"),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct WeatherConfig {
  uri: String,
  app_id: String,
  units: WeatherUnits,
  city_id: String,
}

#[derive(Serialize, Deserialize)]
struct WeatherResponseMain {
  temp: f32,
  feels_like: f32,
  pressure: i32,
  humidity: i32,
}

#[derive(Serialize, Deserialize)]
struct WeatherResponse {
  main: WeatherResponseMain,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AppConfig {
  callback_url: String,
  weather: WeatherConfig,
}

#[tauri::command]
fn login(
  rx: tauri::State<Rx>,
  google_client: tauri::State<Mutex<Client>>,
  window: Window,
) -> Result<(), errors::DashboardError> {
  println!("Called tauri login");

  //We'll just assume that if there was an error opening the file that it doesn't exist
  let auth_url = auth::get_auth_url("http://localhost:8000/callback".to_string())?;
  shell::open(&window.shell_scope(), auth_url, None)
    .map_err(|e| errors::DashboardError::new(format!("{}", e), None))?;
  let auth_code = rx.0.lock().unwrap().recv()?;
  let token = auth::exchange_token(auth_code, "http://localhost:8000/callback".to_string())?;
  auth::save_token(&token, None)?;
  google_client.lock()?.set_token(Some(token));
  Ok(())
}

#[tauri::command]
fn get_weather(
  weather_config: tauri::State<Mutex<WeatherConfig>>,
) -> Result<WeatherResponse, errors::DashboardError> {
  //let uri = "https://api.openweathermap.org/data/2.5/weather";
  let conf = weather_config.lock()?;
  let resp: WeatherResponse = ureq::get(conf.uri.as_str())
    .query("id", conf.city_id.as_str())
    .query("appid", conf.app_id.as_str())
    .query("units", conf.units.to_string().as_str())
    .call()?
    .into_json()?;

  Ok(resp)
}

#[tauri::command]
fn get_calendar(
  google_client: tauri::State<Mutex<Client>>,
) -> Result<CalendarList, errors::DashboardError> {
  google_client.lock()?.list_calendars()
}

#[tauri::command]
fn get_events(
  google_client: tauri::State<Mutex<Client>>,
  config: tauri::State<Mutex<State>>,
) -> Result<EventList, errors::DashboardError> {
  match &config.lock()?.read() {
    Ok(id) => google_client.lock()?.list_events(id.clone().calendar_id),
    Err(e) => Err(errors::DashboardError::new(
      String::from("No calendar id set, please select one in the configuration"),
      Some(format!("{}", e)),
    )),
  }
}

#[tauri::command]
fn save_config(
  calendar_id: String,
  config: tauri::State<Mutex<State>>,
) -> Result<CalendarConfig, errors::DashboardError> {
  let use_config = config.lock()?;
  println!("calendar id {}", &calendar_id);
  match use_config.write(calendar_id.clone()) {
    Ok(_) => Ok(CalendarConfig { calendar_id }),
    Err(e) => Err(errors::DashboardError::new(
      format!("{}", e),
      Some(String::from("saving_config")),
    )),
  }
}

#[tauri::command]
fn load_config(
  config: tauri::State<Mutex<State>>,
) -> Result<CalendarConfig, errors::DashboardError> {
  match config.lock()?.read() {
    Ok(cal) => {
      println!("loading config: {:?}", cal);
      Ok(cal)
    }
    Err(e) => Err(DashboardError::new(format!("{}", e), None)),
  }
}

#[rocket::get("/callback?<code>")]
fn callback(code: &str, tx: &rocket::State<Tx>) -> &'static str {
  if let Err(e) = tx.0.lock().unwrap().send(code.to_string()) {
    println!("{}", e);
    return "Unable to process auth code, something went wrong";
  };
  "You can close now"
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let config_file = Config::builder()
    .add_source(config::File::with_name("./config.toml"))
    .add_source(config::Environment::with_prefix("DASH"))
    .build()?;
  let config = config_file.try_deserialize::<AppConfig>()?;

  let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
  let token = auth::load_token().ok();
  let google_client = Client::new(None, token, config.callback_url.clone());
  //let config = config::Config::read()?;

  let state = State::new("./state/db")?;
  tauri::Builder::default()
    .setup(move |_app| {
      tauri::async_runtime::spawn(
        rocket::build()
          .manage(Tx(Mutex::new(tx.clone())))
          .mount("/", rocket::routes![callback])
          .launch(),
      );
      Ok(())
    })
    .manage(Rx(Mutex::new(rx)))
    .manage(Mutex::new(google_client))
    .manage(Mutex::new(state))
    .manage(Mutex::new(config.weather))
    .invoke_handler(tauri::generate_handler![
      get_weather,
      login,
      get_calendar,
      save_config,
      load_config,
      get_events,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  Ok(())
}
