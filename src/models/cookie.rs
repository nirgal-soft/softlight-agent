use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Cookie{
  pub name: String,
  pub value: String,
  pub domain: String,
}
