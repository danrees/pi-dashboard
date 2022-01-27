const GOOGLE_URL: &'static str = "https://www.googleapis.com/calendar/v3";

use crate::auth;
use crate::auth::MyToken;
use crate::errors::DashboardError;
use oauth2::TokenResponse;
use reqwest::blocking::{Client as RClient, ClientBuilder};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use ureq::{Agent, Middleware, MiddlewareNext, Request, Response};

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
}

impl Client {
  pub fn new(url: Option<String>, token: Option<MyToken>, redirect_url: String) -> Self {
    let mut ab = ureq::AgentBuilder::new()
      .timeout_read(Duration::from_secs(5))
      .timeout_write(Duration::from_secs(5));
    //.build();
    if let Some(tk) = token {
      let mw = Auth {
        token: tk,
        redirect_url,
      };
      ab = ab.middleware(AuthMiddleware::new(mw))
    };
    let agent = ab.build();
    match url {
      Some(url) => Client { url, agent },
      None => Client {
        url: GOOGLE_URL.to_string(),
        agent,
      },
    }
  }

  pub fn list_calendars(&self) -> Result<String, DashboardError> {
    println!("called list_calendars");
    let url = format!("{}{}", self.url, "/users/me/calendarList");
    let caller = self.agent.get(url.as_str());
    let response = match caller.call() {
      Ok(resp) => resp,
      Err(e) => return Err(e.into()),
    };
    match response.into_string() {
      Ok(resp) => Ok(resp),
      Err(e) => Err(e.into()),
    }
  }
}

struct Auth {
  token: MyToken,
  redirect_url: String,
}

struct AuthMiddleware(Arc<Mutex<Auth>>);

impl AuthMiddleware {
  fn new(auth: Auth) -> Self {
    AuthMiddleware(Arc::new(Mutex::new(auth)))
  }
}

impl ureq::Middleware for AuthMiddleware {
  fn handle(&self, request: Request, next: MiddlewareNext) -> Result<Response, ureq::Error> {
    println!("in auth middleware");
    // FIXME: unwrap

    let mut auth_holder = self.0.lock().unwrap();
    let response = next.handle(request.set(
      "Authorization",
      format!("Bearer {}", auth_holder.token.access_token().secret()).as_str(),
    ));
    match response {
      Ok(resp) => {
        println!("ok response {:?}", resp);
        if resp.status() == 401 {
          println!("handing ok 401");
          println!("after lock");
          let refreshed =
            auth::refresh_token(&auth_holder.token, auth_holder.redirect_url.clone()).unwrap();
          if let Err(err) = auth::save_token(&refreshed) {
            println!("resp status {}", err);
            return Err(ureq::Error::Status(401, resp));
          };
          auth::save_token(&refreshed);
          auth_holder.token = refreshed;
        }
        Ok(resp)
      }
      Err(ureq::Error::Status(401, resp)) => {
        println!("returned 401 err: {:?}", resp);
        // FIXME: unwrap
        let mut auth_holder = self.0.lock().unwrap();
        let refreshed =
          auth::refresh_token(&auth_holder.token, auth_holder.redirect_url.clone()).unwrap();
        auth::save_token(&refreshed);
        auth_holder.token = refreshed;
        //next.handle(request)
        Err(ureq::Error::Status(401, resp))
      }
      Err(ureq::Error::Status(code, resp)) => {
        println!("{} -> {:?}", code, resp);
        Err(ureq::Error::Status(code, resp))
      }
      Err(e) => Err(e),
    }
  }
}
