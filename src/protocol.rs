use std::convert::{From, TryFrom, TryInto};
use std::fmt;
use std::fmt::{Display, Formatter};
use crate::error::{Error, ProtocolError, ProtocolErrorKind};
pub trait TInputProtocol {
    fn read_bytes(&mut self) -> Result<Vec<u8>, Error>;

    fn read_byte(&mut self) -> Result<u8, Error>;

    fn read_i32(&mut self) -> Result<i32, Error>;
}

pub trait TOutputProtocol {
    fn write_bytes(&mut self, b: &[u8]) -> Result<(), Error>;

    fn write_byte(&mut self, b: u8) -> Result<(), Error>;

    fn write_i32(&mut self, i: i32) -> Result<(), Error>;

    fn flush(&mut self) -> Result<(), Error>;
}


impl<P> TInputProtocol for Box<P>
where
    P: TInputProtocol + ?Sized,
{
    fn read_bytes(&mut self) -> Result<Vec<u8>, Error> {
        (**self).read_bytes()
    }

    fn read_byte(&mut self) -> Result<u8, Error> {
        (**self).read_byte()
    }

    fn read_i32(&mut self) -> Result<i32, Error> {
        (**self).read_i32()
    }

}


impl<P> TOutputProtocol for Box<P>
where
    P: TOutputProtocol + ?Sized,
{
    fn write_bytes(&mut self, b: &[u8]) -> Result<(), Error> {
        (**self).write_bytes(b)
    }

    fn write_byte(&mut self, b: u8) -> Result<(), Error> {
        (**self).write_byte(b)
    }

    fn write_i32(&mut self, i: i32) -> Result<(), Error> {
        (**self).write_i32(i)
    }

    fn flush(&mut self) -> Result<(), Error> {
        (**self).flush()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TType {
    Frame,
    Audio,
}

impl Display for TType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            TType::Frame => write!(f, "Frame"),
            TType::Audio => write!(f, "Audio"),
        }
    }
}

impl From<TType> for u8 {
    fn from(message_type: TType) -> Self {
        match message_type {
            TType::Frame => 0x01,
            TType::Audio => 0x02,
        }
    }
}

impl TryFrom<u8> for TType {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(TType::Frame),
            0x02 => Ok(TType::Audio),
            unkn => Err(Error::Protocol(ProtocolError {
                kind: ProtocolErrorKind::InvalidData,
                message: format!("cannot convert {} to TMessageType", unkn),
            })),
        }
    }
}