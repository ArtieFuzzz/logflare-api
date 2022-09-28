use super::lib::logflare::push;
use super::lib::structs::LogBody;
use warp;

fn reply(status: u16, message: &str) -> impl warp::Reply {
    return warp::http::Response::builder()
        .status(status)
        .body(message.to_string());
}

pub async fn report(body: LogBody) -> Result<impl warp::Reply, warp::Rejection> {
    match push(body).await {
        Ok(_) => Ok(reply(200, "Reported to LogFlare")),
        Err(_) => Ok(reply(500, "Failed to report to LogFlare")),
    }
}
