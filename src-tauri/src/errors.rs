use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Serialize, Deserialize, Debug)]
pub struct DashboardError {
  msg: String,
  from: Option<String>,
}

impl std::error::Error for DashboardError {}

impl DashboardError {
  pub fn new(msg: String, from: Option<String>) -> Self {
    DashboardError { msg, from }
  }
}

impl std::fmt::Display for DashboardError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:?} -> {}", self.from, self.msg)
  }
}

impl From<ureq::Error> for DashboardError {
  fn from(error: ureq::Error) -> DashboardError {
    DashboardError {
      msg: format!("{}", error),
      from: Some("ureq".to_string()),
    }
  }
}

impl From<std::io::Error> for DashboardError {
  fn from(error: std::io::Error) -> DashboardError {
    DashboardError {
      msg: format!("{}", error),
      from: Some("std::io::Error".to_string()),
    }
  }
}

impl From<serde_json::Error> for DashboardError {
  fn from(error: serde_json::Error) -> DashboardError {
    DashboardError {
      msg: format!("{}", error),
      from: Some("serde_json".to_string()),
    }
  }
}

impl From<oauth2::url::ParseError> for DashboardError {
  fn from(error: oauth2::url::ParseError) -> DashboardError {
    DashboardError {
      msg: format!("{}", error),
      from: Some("oauth2::url::ParseError".to_string()),
    }
  }
}

impl From<std::sync::mpsc::RecvError> for DashboardError {
  fn from(error: std::sync::mpsc::RecvError) -> DashboardError {
    DashboardError {
      msg: format!("{}", error),
      from: Some("std::sync::mpsc::RecvError".to_string()),
    }
  }
}

impl<T> From<std::sync::PoisonError<T>> for DashboardError {
  fn from(error: std::sync::PoisonError<T>) -> DashboardError {
    DashboardError {
      msg: format!("{}", error),
      from: Some("std::sync::PoisonError".to_string()),
    }
  }
}
