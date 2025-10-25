use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Cookie{
  pub name: String,
  pub value: String,
  pub domain: String,
}
