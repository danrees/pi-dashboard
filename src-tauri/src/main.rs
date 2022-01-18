#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod auth;
mod errors;

use rocket;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::File;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use tauri::api::shell;

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
fn login(window: tauri::Window, rx: tauri::State<Rx>) -> Result<(), errors::DashboardError> {
  let token_file = File::open("../.saved_token.json");
  let token = match token_file {
    Ok(file) => {
      let token: auth::MyToken = serde_json::from_reader(file)?;
      token
    }
    Err(_) => {
      //We'll just assume that if there was an error opening the file that it doesn't exist
      let auth_url = auth::get_auth_url("http://localhost:8000/callback".to_string())?;
      shell::open(auth_url, None);
      let auth_code = rx.0.lock().unwrap().recv()?;
      let token = auth::exchange_token(auth_code, "http://localhost:8000/callback".to_string())?;
      serde_json::to_writer(File::create("../.saved_token.json")?, &token)?;
      token
    }
  };
  Ok(())
}

#[tauri::command]
fn get_weather() -> Result<WeatherResponse, errors::DashboardError> {
  let appid = env::var("OPEN_WEATHER").unwrap();
  let id = env::var("CITY_ID").unwrap();
  let uri = "https://api.openweathermap.org/data/2.5/weather";
  let resp: WeatherResponse = ureq::get(&uri[..])
    .query("id", &id[..])
    .query("appid", &appid[..])
    .query("units", "metric")
    .call()?
    .into_json()?;

  Ok(resp)
}

#[rocket::get("/callback?<code>")]
fn callback(code: &str, tx: &rocket::State<Tx>) -> &'static str {
  if let Err(e) = tx.0.lock().unwrap().send(code.to_string()) {
    println!("{}", e);
    return "Unable to process auth code, something went wrong";
  };
  "You can close now"
}

fn main() {
  let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
  tauri::Builder::default()
    .setup(move |app| {
      tauri::async_runtime::spawn(
        rocket::build()
          .manage(Tx(Mutex::new(tx.clone())))
          .mount("/", rocket::routes![callback])
          .launch(),
      );
      Ok(())
    })
    .manage(Rx(Mutex::new(rx)))
    .invoke_handler(tauri::generate_handler![get_weather, login])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
