use std::net::SocketAddr;

use crate::protocol::{TInputProtocol, TOutputProtocol};
use crate::error::Error;
pub trait Message: Sized {
    fn encode<T: TOutputProtocol>(&self, cx: &MsgContext, protocol: &mut T) -> Result<(),Error>;
    fn decode<T: TInputProtocol>(cx: &mut MsgContext, protocol: &mut T) -> Result<Self, Error>;
}


#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MsgContext {
 
    /// target
    pub target: Option<SocketAddr>,
}