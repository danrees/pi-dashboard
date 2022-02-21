const GOOGLE_URL: &'static str = "https://www.googleapis.com/calendar/v3";

use crate::auth;
use crate::auth::MyToken;
use crate::errors::DashboardError;
use cached::{stores::TimedCache, Cached};
use chrono::{DateTime, Utc};
use oauth2::TokenResponse;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use ureq::{Agent, Response};

#[derive(Serialize, Deserialize, Debug)]
pub struct Calendar {
  id: String,
  summary: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CalendarList {
  kind: String,
  etag: String,
  items: Vec<Calendar>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
  kind: String,
  etag: String,
  id: String,
  summary: String,
  status: String,
  created: DateTime<Utc>,
  updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventList {
  kind: String,
  etag: String,
  time_zone: String,
  updated: Option<DateTime<Utc>>,
  items: Vec<Event>,
  //location: String,
  //color_id: String,
  //start: EventDate,
  //end: EventDate,
}

pub struct Client {
  url: String,
  redirect_url: String,
  agent: Agent,
  token: Option<MyToken>,
  cache: TimedCache<String, EventList>,
}

impl Client {
  pub fn new(url: Option<String>, token: Option<MyToken>, redirect_url: String) -> Self {
    let ab = ureq::AgentBuilder::new()
      .timeout_read(Duration::from_secs(5))
      .timeout_write(Duration::from_secs(5));
    let cache = TimedCache::with_lifespan(120);
    let agent = ab.build();
    match url {
      Some(url) => Client {
        url,
        agent,
        token,
        redirect_url,
        cache,
      },
      None => Client {
        url: GOOGLE_URL.to_string(),
        agent,
        token,
        redirect_url,
        cache,
      },
    }
  }

  pub fn set_token(&mut self, token: Option<MyToken>) {
    self.token = token;
  }

  fn with_retry(&mut self, method: &str, path: &str) -> Result<Response, ureq::Error> {
    println!("with_retry");
    let response = self
      .agent
      .request(method, path)
      .set(
        "Authorization",
        format!(
          "Bearer {}",
          self.token.as_ref().unwrap().access_token().secret()
        )
        .as_str(),
      )
      .call();
    match response {
      Err(ureq::Error::Status(401, _)) => {
        println!("status 401, trying to refresh");
        match auth::refresh_token(self.token.as_ref().unwrap(), self.redirect_url.clone()) {
          Ok(refresh_token) => {
            let refresh_token2 = refresh_token.clone();
            self.set_token(Some(refresh_token2.clone()));
            self
              .agent
              .request(method, path)
              .set(
                "Authorization",
                format!("Bearer {}", refresh_token2.clone().access_token().secret()).as_str(),
              )
              .call()
          }
          Err(e) => {
            println!("problem: {}", e);
            Err(ureq::Error::Status(
              500,
              ureq::Response::new(500, "internal server error", format!("{}", e).as_str()).unwrap(),
            ))
          }
        }
      }
      Err(e) => Err(e),
      Ok(r) => Ok(r),
    }
  }

  pub fn list_calendars(&mut self) -> Result<CalendarList, DashboardError> {
    println!("called list_calendars");
    if let None = self.token {
      return Err(DashboardError::new(
        String::from("no token set, login first"),
        None,
      ));
    }
    let url = format!("{}{}", self.url, "/users/me/calendarList");
    let response = match self.with_retry("get", url.as_str()) {
      Ok(resp) => resp,
      Err(e) => return Err(e.into()),
    };
    match response.into_json() {
      Ok(resp) => Ok(resp),
      Err(e) => Err(e.into()),
    }
  }

  pub fn list_events(&mut self, calendar_id: String) -> Result<EventList, DashboardError> {
    let url = format!("{}/calendars/{}/events", self.url, calendar_id);
    let response = self.with_retry("get", url.as_str())?;
    match response.into_json::<EventList>() {
      Ok(resp) => {
        self.cache.cache_set(calendar_id, resp.clone());
        Ok(resp)
      }
      Err(e) => Err(e.into()),
    }
  }
}
