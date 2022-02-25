#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod config;
mod errors;
mod google;

use google::auth;
use google::client::{CalendarList, Client, EventList};

use rocket;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use tauri::{api::shell, Manager, Window};

struct Tx(Mutex<Sender<String>>);
struct Rx(Mutex<Receiver<String>>);

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
fn get_weather() -> Result<WeatherResponse, errors::DashboardError> {
  let appid = env::var("OPEN_WEATHER").unwrap();
  let id = env::var("CITY_ID").unwrap();
  let uri = "https://api.openweathermap.org/data/2.5/weather";
  let resp: WeatherResponse = ureq::get(uri)
    .query("id", &id[..])
    .query("appid", &appid[..])
    .query("units", "metric")
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
  config: tauri::State<Mutex<config::Config>>,
) -> Result<EventList, errors::DashboardError> {
  match &config.lock()?.calendar_id {
    Some(id) => google_client.lock()?.list_events(id.clone()),
    None => Err(errors::DashboardError::new(
      String::from("No calendar id set, please select one in the configuration"),
      None,
    )),
  }
}

#[tauri::command]
fn save_config(
  calendar_id: String,
  config: tauri::State<Mutex<config::Config>>,
) -> Result<config::Config, errors::DashboardError> {
  let mut use_config = config.lock()?;
  println!("{}", calendar_id);
  use_config.calendar_id = Some(calendar_id);
  println!("main: {:?}", use_config);
  match use_config.write() {
    Ok(_) => Ok(use_config.clone()),
    Err(e) => Err(errors::DashboardError::new(
      format!("{}", e),
      Some(String::from("saving_config")),
    )),
  }
}

#[tauri::command]
fn load_config(
  config: tauri::State<Mutex<config::Config>>,
) -> Result<config::Config, errors::DashboardError> {
  let unlocked = config.lock()?.clone();
  Ok(unlocked)
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
  let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
  let token = auth::load_token().ok();
  let google_client = Client::new(None, token, "http://localhost:8000/callback".to_string());
  let config = config::Config::read()?;
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
    .manage(Mutex::new(config))
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
