use crate::{config::read, HTTP};
use std::error::Error;

use super::structs::LogBody;

pub async fn push(data: LogBody) -> Result<bool, Box<dyn Error>> {
    let conf = read("./config.yml")?;

    match HTTP
        .post(conf.logflare_url)
        .header("X-API-KEY", conf.logflare_key)
        .json(&data)
        .send()
        .await
    {
        Ok(_res) => Ok(true),
        Err(_why) => Ok(false),
    }
}
