use log::debug;
use oauth2::basic::BasicClient;
use oauth2::ureq::http_client;
use oauth2::{
  AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, RefreshToken, Scope,
  TokenResponse, TokenUrl,
};

use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::time::Duration;

use crate::errors::DashboardError;

pub type MyToken =
  oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>;

impl std::convert::From<SaveCreds> for MyToken {
  fn from(creds: SaveCreds) -> MyToken {
    let mut token = oauth2::StandardTokenResponse::new(
      oauth2::AccessToken::new(creds.access_token),
      oauth2::basic::BasicTokenType::Bearer,
      oauth2::EmptyExtraTokenFields {},
    );
    token.set_refresh_token(creds.refresh_token.map(|o| RefreshToken::new(o)));
    token
  }
}

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

#[derive(Serialize, Deserialize, Debug)]
struct SaveCreds {
  access_token: String,
  refresh_token: Option<String>,
  expires_in: Option<Duration>,
  extra_fields: Option<HashMap<String, String>>,
}

impl std::convert::From<&MyToken> for SaveCreds {
  fn from(token: &MyToken) -> SaveCreds {
    SaveCreds {
      access_token: token.access_token().secret().clone(),
      refresh_token: token.refresh_token().map(|o| o.secret().clone()),
      expires_in: token.expires_in(),
      //TODO: need to understand how the extra fields behaves to know how this might work
      extra_fields: None,
    }
  }
}

fn load_creds() -> Result<OauthCreds, DashboardError> {
  let x: OauthCreds = serde_json::from_reader(File::open("../.desktop_credentials.json")?)?;
  debug!("loaded credentials");
  Ok(x)
}

pub fn save_token(
  token: &MyToken,
  refresh_token: Option<oauth2::RefreshToken>,
) -> Result<(), DashboardError> {
  debug!("saving token");
  let mut to_save: SaveCreds = token.into();
  if let Some(t) = refresh_token {
    to_save.refresh_token = Some(t.secret().clone());
  }
  match serde_json::to_writer(File::create("../.saved_token.json")?, &to_save) {
    Ok(r) => Ok(r),
    Err(e) => Err(e.into()),
  }
}

pub fn load_token() -> Result<MyToken, DashboardError> {
  let token: SaveCreds = serde_json::from_reader(File::open("../.saved_token.json")?)?;
  Ok(token.into())
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

  let (auth_url, _csrf_token) = client
    .authorize_url(CsrfToken::new_random)
    .add_scope(Scope::new(
      "https://www.googleapis.com/auth/calendar".to_string(),
    ))
    .url();
  Ok(auth_url.into())
}

pub fn exchange_token(auth_code: String, redirect_url: String) -> Result<MyToken, DashboardError> {
  debug!("auth code: {}", auth_code);
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

pub fn refresh_token(token: &MyToken, redirect_url: String) -> Result<MyToken, DashboardError> {
  debug!("refreshing token");
  let creds = load_creds()?;
  let client = BasicClient::new(
    ClientId::new(creds.installed.client_id),
    Some(ClientSecret::new(creds.installed.client_secret)),
    AuthUrl::new(creds.installed.auth_uri)?,
    Some(TokenUrl::new(creds.installed.token_uri)?),
  )
  .set_redirect_uri(RedirectUrl::new(redirect_url)?);
  if let Some(refresh_token) = token.refresh_token() {
    debug!("Refresh token is: {}", refresh_token.secret());
    let new_token = client
      .exchange_refresh_token(refresh_token)
      .request(http_client)
      .or_else(|e| {
        return Err(DashboardError::new(
          format!("{:?}", e),
          Some("oauth2::RequestTokenError".to_string()),
        ));
      });
    match &new_token {
      Ok(t) => {
        // FIXME: I don't really like having to save and load the token to avoid ownership issues
        save_token(t, Some(refresh_token.clone()))?;
        load_token()
      }
      Err(e) => Err(DashboardError::new(
        String::from("unable to refresh token, please do so manually"),
        Some(format!("{}", e)),
      )),
    }
  } else {
    Err(DashboardError::new(
      String::from("No refresh token set"),
      None,
    ))
  }
}
