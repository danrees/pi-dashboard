use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
  pub calendar_id: Option<String>,
}

impl Config {
  pub fn write(&self) -> Result<(), Box<dyn std::error::Error>> {
    let val = toml::to_string(self)?;
    println!("here: {:?}", self);
    if let Err(e) = fs::write("./state.toml", val) {
      Err(Box::new(e))
    } else {
      Ok(())
    }
  }

  pub fn read() -> Result<Config, Box<dyn std::error::Error>> {
    let path = Path::new("./state.toml");
    if !path.exists() {
      return Ok(Config { calendar_id: None });
    }
    let val = fs::read_to_string("./state.toml")?;
    let contents = toml::from_str(val.as_str())?;
    Ok(contents)
  }
}
