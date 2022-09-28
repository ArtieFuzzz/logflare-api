use std::fs::File;
use serde::{Deserialize, Serialize};
use serde_yaml::{from_reader, Error};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub host: String,
    pub port: String,
    #[serde(rename = "logflareKey")]
    pub logflare_key: String,
    #[serde(rename = "logflareUrl")]
    pub logflare_url: String,
}

pub fn read<'a>(path: &'a str) -> Result<Configuration, Error> {
    let reader = File::open(path).unwrap();

    let conf: Result<Configuration, _> = from_reader(reader);

    return conf;
}
