use std::net::SocketAddr;

use ansi_term::Color::Green;
use color_eyre::Result;
use once_cell::sync::Lazy;
use reqwest::Client;
use tracing::log::warn;
use warp::{self, Filter};

mod config;
mod server;

use server::lib::structs::LogBody;

pub static HTTP: Lazy<Client> = Lazy::new(Client::new);

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    println!(
        "{}",
        r"
     _                _____ _
    | |    ___   __ _|  ___| | __ _ _ __ ___
    | |   / _ \ / _` | |_  | |/ _` | '__/ _ \
    | |__| (_) | (_| |  _| | | (_| | | |  __/
    |_____\___/ \__, |_|   |_|\__,_|_|  \___|
                |___/
    "
    );

    println!(
        "{}\n",
        Green.paint(format!(
            "API Interface Written in Rust, V{}",
            env!("CARGO_PKG_VERSION")
        ))
    );

    let conf = config::read("./config.yml")?;
    let addr = format!("{}:{}", conf.host, conf.port).parse::<SocketAddr>()?;

    let report = warp::path!("report")
        .and(warp::post())
        .and(warp::body::json::<LogBody>())
        .and_then(server::routes::report);

    let routes = warp::any().and(report);

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not set CTRL-C handler");
        warn!("Received Termination Signal...");
        std::process::exit(0)
    });

    warp::serve(routes).run(addr).await;

    Ok(())
}
