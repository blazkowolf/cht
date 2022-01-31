mod cht;
mod config;
mod error;

use std::env;
use std::io;
use std::io::Write;

use hyper::{body::HttpBody as _, Body, Response};

use cht::ChtClient;
use config::Config;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args)?;
    let mut res: Response<Body> = ChtClient::default().cheat(config).await?;

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk)?;
    }

    Ok(())
}
