#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod auth;
mod errors;

use rocket;
use serde::{Deserialize, Serialize};
use std::env;

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
fn get_auth_url() -> Result<String, errors::DashboardError> {
  auth::get_auth_url(String::from("http://localhost:8000/callback"))
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
fn callback(code: &str) -> &'static str {
  match auth::exchange_token(
    String::from(code),
    String::from("http://localhost:8000/callback"),
  ) {
    Err(e) => println!("{}", e),
    _ => {}
  };
  "You can close now"
}

fn main() {
  tauri::Builder::default()
    .on_page_load(|window, page_load| {
      println!("Opening window: {}", window.label());
    })
    .setup(|app| {
      tauri::async_runtime::spawn(
        rocket::build()
          .mount("/", rocket::routes![callback])
          .launch(),
      );
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![get_weather, get_auth_url])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
