use std::{error::Error, net::TcpListener};
use rust_pulse::configurations;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = configurations::get_configurations()?;
    let listener = TcpListener::bind(format!("{}:{}", settings.get_host(), settings.get_application_port()))?;
    rust_pulse::run(listener).await?.await?;

    Ok(())
}
