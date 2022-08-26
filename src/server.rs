use std::net::SocketAddr;
use std::io;
use futures::{Stream, Sink, Future};
use flume::{Receiver, Sender};
use tokio_stream::wrappers::TcpListenerStream;
use tokio_util::codec::{Decoder, Framed};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;

use crate::mirror::RemoteDesktopService;
use crate::error::Error;

#[async_trait::async_trait]
pub trait Listenable {
    type Conn: AsyncRead + AsyncWrite + Send + Unpin + 'static;
    type Stream: Stream<Item = io::Result<Self::Conn>> + Unpin;

    async fn bind(&self) -> io::Result<Self::Stream>;
}

#[async_trait::async_trait]
impl Listenable for SocketAddr {
    type Conn = TcpStream;
    type Stream = TcpListenerStream;

    async fn bind(&self) -> io::Result<Self::Stream> {
        let listener = tokio::net::TcpListener::bind(self).await?;
        Ok(TcpListenerStream::new(listener))
    }
}


pub async fn run(listener: SocketAddr, shutdown: impl Future) -> Result<(), Error> {
    let (mut shutdown_complete_tx, mut shutdown_complete_rx) = flume::bounded::<()>(1);
    let server = Server {
        tcp_listener: listener.bind().await?,
        remote_desk_service: RemoteDesktopService::new(shutdown_complete_rx)?,
        shutdown_complete_tx,
    };

    Ok(())
} 


pub struct Server {
    tcp_listener: TcpListenerStream,
    remote_desk_service: RemoteDesktopService,
    shutdown_complete_tx: Sender<()>,
}

impl Server {
    pub async fn run() {

    }
}
