use futures::{Stream, Sink, Future};
use flume::{Receiver, Sender};
use tokio;
use tokio_util::codec::{Decoder, Framed};
use tokio::io::{AsyncRead, AsyncWrite};


pub trait IntoStaticFuture {
    type Future: Future<Item = Self::Item, Error = Self::Error> + 'static + Send;
    type Item;
    type Error;

    fn into_static_future(self) -> Self::Future;
}

impl<F: IntoFuture> IntoStaticFuture for F
where
    <F as IntoFuture>::Future: 'static + Send,
{
    type Future = <F as IntoFuture>::Future;
    type Item = <F as IntoFuture>::Item;
    type Error = <F as IntoFuture>::Error;

    fn into_static_future(self) -> Self::Future {
        self.into_future()
    }
}

/* struct Transport<T: AsyncRead + AsyncWrite>(Framed<T, Codec>);

impl<T> Stream for Transport<T>
where
    T: AsyncRead + AsyncWrite,
{
    type Item = Message;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        trace!("Transport: polling");
        self.0.poll()
    }
}

impl<T> Sink for Transport<T>
where
    T: AsyncRead + AsyncWrite,
{
    type SinkItem = Message;
    type SinkError = io::Error;

    fn start_send(&mut self, item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        self.0.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.0.poll_complete()
    }
}

trait MessageHandler {
    // We just received `msg` on our input stream. Handle it.
    fn handle_incoming(&mut self, msg: Message);

    // Try to push out all of the outgoing messages (e.g. responses in the case of a server,
    // notifications+requests in the case of a client) onto the sink. Return Ok(Async::Ready(()))
    // if we managed to push them all out and flush the sink.
    fn send_outgoing<T: AsyncRead + AsyncWrite>(
        &mut self,
        sink: &mut Transport<T>,
    ) -> Poll<(), io::Error>;

    // Is the endpoint finished? This is only relevant for clients, since servers and
    // client+servers will never voluntarily stop.
    fn is_finished(&self) -> bool {
        false
    }
} */

pub struct Server {
  
}