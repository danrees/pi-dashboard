const GOOGLE_URL: &'static str = "https://www.googleapis.com/calendar/v3";

use crate::auth::MyToken;
use crate::errors::DashboardError;
use oauth2::TokenResponse;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use ureq::Agent;

#[derive(Serialize, Deserialize, Debug)]
pub struct Calendar {
  pub kind: String,
  pub etag: String,
  pub id: String,
  pub summary: String,
  pub location: String,
  pub time_zone: String,
}

pub struct Client {
  url: String,
  agent: Agent,
  token: Option<MyToken>,
}

impl Client {
  pub fn new(url: Option<String>, token: Option<MyToken>, agent: Agent) -> Self {
    match url {
      Some(url) => Client { url, token, agent },
      None => Client {
        url: GOOGLE_URL.to_string(),
        token,
        agent,
      },
    }
  }

  pub fn list_calendars(&self) -> Result<String, DashboardError> {
    if let Some(token) = &self.token {
      let url = format!("{}{}", self.url, "/users/me/calendarList");
      println!("{}", token.access_token().secret());
      match self
        .agent
        .get(url.as_str())
        .set(
          "Authorization",
          format!("Bearer {}", token.access_token().secret()).as_str(),
        )
        .call()?
        .into_string()
      {
        Ok(result) => Ok(result),
        Err(e) => Err(e.into()),
      }
    } else {
      Err(DashboardError::new(String::from("Token is unset"), None))
    }
  }

  pub fn update_token(&mut self, token: MyToken) {
    self.token = Some(token);
  }
}

impl Deref for Client {
  type Target = Option<MyToken>;
  fn deref(&self) -> &Option<MyToken> {
    &self.token
  }
}

impl DerefMut for Client {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.token
  }
}
