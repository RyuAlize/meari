use std::marker::PhantomData;
use tokio_util::codec::{Decoder, Encoder};
use bytes::{BytesMut, BufMut, Buf};
use crate::message::{Message, MsgContext};
use crate::protocol::{TInputProtocol, TOutputProtocol, TType};
use crate::error::{Error, ProtocolErrorKind, new_protocol_error};

pub struct FramedCodec<C>(C);

impl<C> FramedCodec<C> {
    #[allow(dead_code)]
    pub fn new(c: C) -> Self {
        FramedCodec(c)
    }
}

const MAX_MESSAGE_SIZE: usize = 16 * 1024 * 1024;

impl<C, T> Encoder<T> for FramedCodec<C> 
where
    C: Encoder<T>,
    crate::error::Error: From<C::Error>,
{
    type Error = crate::error::Error;

    fn encode(&mut self, item: T, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        let zero_index = dst.len();
        dst.reserve(4);
        unsafe {
            dst.advance_mut(4);
        }
        self.0.encode(item, dst).map_err(Into::into)?;
        let written = dst.len() - 4 - zero_index;
        if written > MAX_MESSAGE_SIZE {
            return Err(new_protocol_error(
                ProtocolErrorKind::SizeLimit,
                format!("Frame of length {} is too large.", written),
            ));
        }
        let mut buf = &mut dst[zero_index..zero_index + 4];
        buf.put_u32(written as u32);
        Ok(())
    }
}

impl<C> Decoder for FramedCodec<C>
where
    C: Decoder,
    crate::error::Error: From<C::Error>,
{
    type Item = C::Item;
    type Error = crate::error::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 {
            return Ok(None);
        }
        let mut length_bytes = [0u8; 4];
        length_bytes.copy_from_slice(&src[..4]);
        let length = u32::from_be_bytes(length_bytes) as usize;

        if length > MAX_MESSAGE_SIZE {
            return Err(new_protocol_error(
                ProtocolErrorKind::SizeLimit,
                format!("Frame of length {} is too large.", length),
            ));
        }

        if src.len() < 4 + length {
            src.reserve(4 + length - src.len());
            return Ok(None);
        }

        src.advance(4);
        let decoded = self.0.decode(src)?;
        match decoded {
            None => Err(new_protocol_error(
                ProtocolErrorKind::InvalidData,
                "unable to decode message which the data size is enough for decoding",
            )),
            Some(inner) => Ok(Some(inner)),
        }
    }
}


#[derive(Debug)]
pub struct MessageCodec<E, D> {
    strict: bool,
    _phantom: PhantomData<fn(E, D)>,
}

impl<E, D> MessageCodec<E, D> {
    #[allow(unused)]
    pub fn new(strict: bool) -> Self {
        Self {
            strict,
            _phantom: PhantomData,
        }
    }
}

impl<E, D> Encoder<(MsgContext, E)> for MessageCodec<E, D>
where
    E: Message,
{
    type Error = crate::error::Error;

    fn encode(
        &mut self,
        item: (MsgContext, E),
        dst: &mut BytesMut,
    ) -> Result<(), Self::Error> {
        let mut protocol = TBinaryOutputProtocol::new(dst, self.strict);
        let (mut cx, it) = item;
        it.encode(&cx, &mut protocol)?;
        Ok(())
    }
}

impl<E, D> Decoder for MessageCodec<E, D>
where
    D: Message,
{
    type Item = (MsgContext, D);
    type Error = crate::error::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // We hard code BinaryProtocol for now. TODO: fix it
        let mut protocol = TBinaryInputProtocol::new(src, self.strict);
        let mut cx = MsgContext::default();

       
        let item = D::decode(&mut cx, &mut protocol)?;
        Ok(Some((cx, item)))
    }
}

#[derive(Debug)]
pub struct TBinaryInputProtocol<T> {
    buf: T,
    strict: bool,
}

impl<T> TBinaryInputProtocol<T> {
    #[allow(unused)]
    pub fn new(buf: T, strict: bool) -> TBinaryInputProtocol<T> {
        TBinaryInputProtocol { buf, strict }
    }
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

impl<T> TBinaryOutputProtocol<T> {
    #[allow(dead_code)]
    pub fn new(buf: T, strict: bool) -> TBinaryOutputProtocol<T> {
        TBinaryOutputProtocol { buf, strict }
    }
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
