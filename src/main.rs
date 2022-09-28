use std::net::SocketAddr;

use ansi_term::Color::Green;
use color_eyre::Result;
use neofiglet::FIGfont;
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

    let figlet = FIGfont::standard().unwrap();
    println!("{}\n", figlet.convert("LogFlare Backend").unwrap());
    println!("{}\n", Green.paint("Written in Rust"));

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
