use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogBody {
  pub level: String,
  pub fatal: bool,
  pub message: String,
  pub tags: Vec<String>,
}
