use oauth2::basic::BasicClient;
use oauth2::ureq::http_client;
use oauth2::{
  AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;

use crate::errors::DashboardError;

pub type MyToken =
  oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>;

#[derive(Serialize, Deserialize)]
struct InstalledCreds {
  client_id: String,
  auth_uri: String,
  token_uri: String,
  client_secret: String,
  redirect_uris: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct OauthCreds {
  installed: InstalledCreds,
}

fn load_creds() -> Result<OauthCreds, DashboardError> {
  let x: OauthCreds = serde_json::from_reader(File::open("../.desktop_credentials.json")?)?;
  Ok(x)
}

pub fn get_auth_url(redirect_url: String) -> Result<String, DashboardError> {
  let creds = load_creds()?;
  let client = BasicClient::new(
    ClientId::new(creds.installed.client_id),
    Some(ClientSecret::new(creds.installed.client_secret)),
    AuthUrl::new(creds.installed.auth_uri)?,
    Some(TokenUrl::new(creds.installed.token_uri)?),
  )
  .set_redirect_uri(RedirectUrl::new(redirect_url)?);

  let (auth_url, csrf_token) = client
    .authorize_url(CsrfToken::new_random)
    .add_scope(Scope::new(
      "https://www.googleapis.com/auth/calendar".to_string(),
    ))
    .url();
  Ok(auth_url.into())
}

pub fn exchange_token(auth_code: String, redirect_url: String) -> Result<MyToken, DashboardError> {
  println!("Code: {}", auth_code);
  let creds = load_creds()?;
  let client = BasicClient::new(
    ClientId::new(creds.installed.client_id),
    Some(ClientSecret::new(creds.installed.client_secret)),
    AuthUrl::new(creds.installed.auth_uri)?,
    Some(TokenUrl::new(creds.installed.token_uri)?),
  )
  .set_redirect_uri(RedirectUrl::new(redirect_url)?);
  let token = client
    .exchange_code(AuthorizationCode::new(auth_code))
    .request(http_client)
    .or_else(|e| {
      return Err(DashboardError::new(
        format!("{}", e),
        Some("oauth2::RequestTokenError".to_string()),
      ));
    });
  token
}

pub fn get_token() -> Option<MyToken> {
  let reader = File::open("../.saved_token.json").ok();
  if let Some(r) = reader {
    serde_json::from_reader(r).ok()
  } else {
    None
  }
}
