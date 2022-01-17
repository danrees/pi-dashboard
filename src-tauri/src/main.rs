#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod errors;

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

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_weather])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
