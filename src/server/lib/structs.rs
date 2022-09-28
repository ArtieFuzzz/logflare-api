use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogBody {
  pub message: String,
  pub metadata: Metadata
}

/* 
  stacktrace: Not required, only for when we have an Error or a Fatal Error
  fatal: Is this log a fatal error?
  level: Defines the level of this log
  from: Who was this from?
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
  pub stacktrace: Option<String>,
  pub fatal: bool,
  pub level: String,
  pub from: String,
}
