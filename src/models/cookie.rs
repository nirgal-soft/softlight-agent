use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookie{
  pub name: String,
  pub value: String,
  pub domain: String,
}
