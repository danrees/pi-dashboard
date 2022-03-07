use crate::errors::DashboardError;
use serde::{Deserialize, Serialize};

const CALENDAR_ID: &'static str = "calendar_id";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
  pub calendar_id: String,
}

pub struct State {
  db: sled::Db,
}

impl State {
  pub fn new(dir: &str) -> Result<Self, Box<dyn std::error::Error>> {
    let db = sled::open(dir)?;
    Ok(State { db })
  }

  pub fn write(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
    self.db.insert(CALENDAR_ID, id.as_bytes())?;
    Ok(())
  }

  pub fn read(&self) -> Result<Config, Box<dyn std::error::Error>> {
    let answer = self
      .db
      .get(CALENDAR_ID)?
      .ok_or(DashboardError::new("no calendar id found".into(), None))?;
    let answer2 = std::str::from_utf8(&answer)?;
    Ok(Config {
      calendar_id: answer2.into(),
    })
  }
}
