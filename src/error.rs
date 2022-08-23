use std::io;
use std::fmt::{self, Formatter, Display};
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO Error")]
    IOError(#[from] io::Error),
    
    #[error("Transport Error")]
    Transport(TransportError),

    #[error("Protocol Error")]
    Protocol(ProtocolError)
}


#[derive(Debug, Eq, PartialEq)]
pub struct TransportError {  
    pub kind: TransportErrorKind,
    pub message: String,
}

impl TransportError {
    pub fn new<S: Into<String>>(kind: TransportErrorKind, message: S) -> TransportError {
        TransportError {
            kind,
            message: message.into(),
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Copy, Eq, Debug, PartialEq)]
pub enum TransportErrorKind {
    /// Catch-all I/O error.
    Unknown = 0,
    /// An I/O operation was attempted when the transport channel was not open.
    NotOpen = 1,
    /// The transport channel cannot be opened because it was opened previously.
    AlreadyOpen = 2,
    /// An I/O operation timed out.
    TimedOut = 3,
    /// A read could not complete because no bytes were available.
    EndOfFile = 4,
    /// An invalid (buffer/message) size was requested or received.
    NegativeSize = 5,
    /// Too large a buffer or message size was requested or received.
    SizeLimit = 6,
}

impl Display for TransportError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let error_text = match self.kind {
            TransportErrorKind::Unknown => "transport error",
            TransportErrorKind::NotOpen => "not open",
            TransportErrorKind::AlreadyOpen => "already open",
            TransportErrorKind::TimedOut => "timed out",
            TransportErrorKind::EndOfFile => "end of file",
            TransportErrorKind::NegativeSize => "negative size message",
            TransportErrorKind::SizeLimit => "message too long",
        };

        write!(f, "{}", error_text)
    }
}

impl TryFrom<i32> for TransportErrorKind {
    type Error = Error;
    fn try_from(from: i32) -> Result<Self, Self::Error> {
        match from {
            0 => Ok(TransportErrorKind::Unknown),
            1 => Ok(TransportErrorKind::NotOpen),
            2 => Ok(TransportErrorKind::AlreadyOpen),
            3 => Ok(TransportErrorKind::TimedOut),
            4 => Ok(TransportErrorKind::EndOfFile),
            5 => Ok(TransportErrorKind::NegativeSize),
            6 => Ok(TransportErrorKind::SizeLimit),
            _ => Err(Error::Protocol(ProtocolError {
                kind: ProtocolErrorKind::Unknown,
                message: format!("cannot convert {} to TransportErrorKind", from),
            })),
        }
    }
}

pub fn new_protocol_error<S: Into<String>>(kind: ProtocolErrorKind, message: S) -> Error {
    Error::Protocol(ProtocolError::new(kind, message))
}

#[derive(Debug, Eq, PartialEq)]
pub struct ProtocolError {
    pub kind: ProtocolErrorKind,
    pub message: String,
}

impl ProtocolError {
    pub fn new<S: Into<String>>(kind: ProtocolErrorKind, message: S) -> ProtocolError {
        ProtocolError {
            kind,
            message: message.into(),
        }
    }
}

#[non_exhaustive]
#[derive(Clone, Copy, Eq, Debug, PartialEq)]
pub enum ProtocolErrorKind {
    /// Catch-all runtime-library error.
    Unknown = 0,
    /// An invalid argument was supplied to a library function, or invalid data was received.
    InvalidData = 1,
    /// An invalid size was received in an encoded field.
    NegativeSize = 2,
    /// Thrift message or field was too long.
    SizeLimit = 3,
    /// Unsupported or unknown Thrift protocol version.
    BadVersion = 4,
    /// Unsupported Thrift protocol, server or field type.
    NotImplemented = 5,

}

impl Display for ProtocolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let error_text = match self.kind {
            ProtocolErrorKind::Unknown => "protocol error",
            ProtocolErrorKind::InvalidData => "bad data",
            ProtocolErrorKind::NegativeSize => "negative message size",
            ProtocolErrorKind::SizeLimit => "message too long",
            ProtocolErrorKind::BadVersion => "invalid thrift version",
            ProtocolErrorKind::NotImplemented => "not implemented",
        };

        write!(f, "{}", error_text)
    }
}

impl TryFrom<i32> for ProtocolErrorKind {
    type Error = Error;
    fn try_from(from: i32) -> Result<Self, Self::Error> {
        match from {
            0 => Ok(ProtocolErrorKind::Unknown),
            1 => Ok(ProtocolErrorKind::InvalidData),
            2 => Ok(ProtocolErrorKind::NegativeSize),
            3 => Ok(ProtocolErrorKind::SizeLimit),
            4 => Ok(ProtocolErrorKind::BadVersion),
            5 => Ok(ProtocolErrorKind::NotImplemented),
            _ => Err(Error::Protocol(ProtocolError {
                kind: ProtocolErrorKind::Unknown,
                message: format!("cannot convert {} to ProtocolErrorKind", from),
            })),
        }
    }
}