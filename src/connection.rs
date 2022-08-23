use std::net::SocketAddr;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use tower::Service;
use tokio::net::TcpListener;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

use crate::util::BoxFuture;

pub trait Io: AsyncWrite + AsyncRead + Send + 'static {}

impl<T> Io for T where T: AsyncRead + AsyncWrite + Send + 'static {}

pub struct BoxedIo(Pin<Box<dyn Io>>);

impl BoxedIo {
    pub fn new<I: Io>(io: I) -> BoxedIo {
        BoxedIo(Box::pin(io))
    }
}

impl AsyncRead for BoxedIo {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.0).poll_read(cx, buf)
    }
}

impl AsyncWrite for BoxedIo {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.0).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.0).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.0).poll_shutdown(cx)
    }
}

#[derive(Debug, Clone)]
pub struct DefaultMakeConnection;


impl Service<SocketAddr> for DefaultMakeConnection {
    type Response = tokio::net::TcpStream;
    type Error = std::io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: SocketAddr) -> Self::Future {
        Box::pin(tokio::net::TcpStream::connect(req))
    }
}
