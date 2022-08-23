
use tokio_util::codec::{Decoder, Encoder};

use crate::protocol::{TInputProtocol, TOutputProtocol};
use crate::error::{Error, ProtocolErrorKind, new_protocol_error};


pub struct Codec<C>(C);

#[derive(Debug)]
pub struct TBinaryInputProtocol<T> {
    buf: T,
    strict: bool,
}

impl<T> TInputProtocol for TBinaryInputProtocol<T>
where
    T: bytes::Buf,
{
    #[inline]
    fn read_bytes(&mut self) -> Result<Vec<u8>, Error> {
        protocol_len_check(&self.buf, 4)?;
        let num_bytes = self.buf.get_i32() as usize;
        let mut output = vec![0; num_bytes];
        protocol_len_check(&self.buf, num_bytes)?;
        self.buf.copy_to_slice(&mut output);
        Ok(output)
    }

    #[inline]
    fn read_byte(&mut self) -> Result<u8, Error> {
        protocol_len_check(&self.buf, 1)?;
        Ok(self.buf.get_u8())
    }

    #[inline]
    fn read_i32(&mut self) -> Result<i32, Error> {
        protocol_len_check(&self.buf, 4)?;
        Ok(self.buf.get_i32())
    }
}

#[derive(Debug)]
pub struct TBinaryOutputProtocol<T> {
    buf: T,
    strict: bool,
}

impl<T> TOutputProtocol for TBinaryOutputProtocol<T>
where
    T: bytes::BufMut,
{
    #[inline]
    fn write_bytes(&mut self, b: &[u8]) -> Result<(), Error> {
        self.write_i32(b.len() as i32)?;
        self.buf.put_slice(b);
        Ok(())
    }

    #[inline]
    fn write_byte(&mut self, b: u8) -> Result<(), Error> {
        self.buf.put_u8(b);
        Ok(())
    }

    #[inline]
    fn write_i32(&mut self, i: i32) -> Result<(), Error> {
        self.buf.put_i32(i);
        Ok(())
    }

    
    #[inline]
    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }

}


#[inline]
fn protocol_len_check<T>(buf: &T, required_len: usize) -> Result<(), Error>
where
    T: bytes::Buf,
{
    #[cfg(not(feature = "unstable"))]
    if buf.remaining() >= required_len {
        return Ok(());
    }
    #[cfg(feature = "unstable")]
    if std::intrinsics::likely(buf.remaining() >= required_len) {
        return Ok(());
    }
    Err(new_protocol_error(
        ProtocolErrorKind::InvalidData,
        "unexpected data length",
    ))
}
