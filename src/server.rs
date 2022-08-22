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



pub struct Server {
    
}