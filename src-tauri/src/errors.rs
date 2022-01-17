use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Serialize, Deserialize)]
pub struct DashboardError(pub String);

impl std::fmt::Display for DashboardError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}

impl From<ureq::Error> for DashboardError {
  fn from(error: ureq::Error) -> DashboardError {
    DashboardError(format!("{}", error))
  }
}

impl From<std::io::Error> for DashboardError {
  fn from(error: std::io::Error) -> DashboardError {
    DashboardError(format!("{}", error))
  }
}

impl From<serde_json::Error> for DashboardError {
  fn from(error: serde_json::Error) -> DashboardError {
    DashboardError(format!("{}", error))
  }
}

impl From<oauth2::url::ParseError> for DashboardError {
  fn from(error: oauth2::url::ParseError) -> DashboardError {
    DashboardError(format!("{}", error))
  }
}
