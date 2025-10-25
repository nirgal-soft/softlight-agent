use std::collections::HashMap;
use serde::Deserialize;
use crate::models::cookie::Cookie;

#[derive(Debug, Deserialize)]
pub struct Setup{
  pub auth_required: bool,
  pub starting_url: Option<String>,
  #[serde(default)]
  pub cookies: Option<Vec<Cookie>>,
  #[serde(default)]
  pub local_storage: Option<HashMap<String, String>>,
}
