use std::sync::Arc;

use anyhow::Result;
use geek_chatserver::{handle_client, PublicChannel};
use tokio::net::TcpListener;
use tracing::{level_filters::LevelFilter, warn};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().pretty().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;

    let public_channel = Arc::new(PublicChannel::default());
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Accepted connection from {}", addr);
        let channel = public_channel.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(channel, stream, addr).await {
                warn!("Error handling connection: {}", e)
            }
        });
    }

    #[allow(unreachable_code)]
    Ok(())
}
