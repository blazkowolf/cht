mod cht;
mod config;
mod error;

use std::env;
use std::error::Error;
use std::process;

use cht::Cht;
use config::Config;

use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt as _};

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = Cht::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
