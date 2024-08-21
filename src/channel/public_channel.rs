use anyhow::Result;
use dashmap::DashMap;
use futures::{stream::SplitSink, SinkExt, StreamExt};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LinesCodec};
use tracing::warn;

use crate::message::Message;

#[derive(Debug, Default)]
pub struct PublicChannel {
    // peer: DashMap<SocketAddr, mpsc::Sender<Arc<Message>>>,
    peer: DashMap<SocketAddr, SplitSink<Framed<TcpStream, LinesCodec>, String>>,
}

impl PublicChannel {
    async fn broadcast(&self, addr: SocketAddr, message: Arc<Message>) {
        for mut peer in self.peer.iter_mut() {
            if peer.key() == &addr {
                continue;
            }
            let sender = peer.value_mut();
            if let Err(e) = sender.send(message.clone().to_string()).await {
                warn!("Failed to send message to peer: {}", e);
                self.peer.remove(peer.key());
            }
        }
    }
}

pub async fn handle_client(
    channel: Arc<PublicChannel>,
    stream: TcpStream,
    addr: SocketAddr,
) -> Result<()> {
    let stream = Framed::new(stream, LinesCodec::new());

    let (mut stream_sender, mut stream_receiver) = stream.split();

    stream_sender
        .send("Enter your username:".to_string())
        .await?;

    let username = match stream_receiver.next().await {
        Some(Ok(username)) => username,
        Some(Err(e)) => return Err(e.into()),
        None => return Ok(()),
    };

    channel.peer.insert(addr, stream_sender);

    let message = Arc::new(Message::user_joined(&username));
    channel.broadcast(addr, message).await;

    while let Some(line) = stream_receiver.next().await {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                warn!("Failed to receive message from client: {}", e);
                break;
            }
        };

        let message = Arc::new(Message::chat(&username, line));
        channel.broadcast(addr, message).await;
    }

    channel.peer.remove(&addr);
    let message = Arc::new(Message::user_left(&username));
    channel.broadcast(addr, message).await;

    Ok(())
}
