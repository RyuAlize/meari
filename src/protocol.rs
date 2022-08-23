use std::convert::{From, TryFrom, TryInto};
use std::fmt;
use std::fmt::{Display, Formatter};

pub trait TInputProtocol {
    fn read_bytes(&mut self) -> crate::Result<Vec<u8>>;

    fn read_i32(&mut self) -> crate::Result<i32>;
}

pub trait TOutputProtocol {
    fn write_bytes(&mut self, b: &[u8]) -> crate::Result<()>;

    fn write_i32(&mut self, i: i32) -> crate::Result<()>;

    fn flush(&mut self) -> crate::Result<()>;
}


impl<P> TInputProtocol for Box<P>
where
    P: TInputProtocol + ?Sized,
{
    fn read_bytes(&mut self) -> crate::Result<Vec<u8>> {
        (**self).read_bytes()
    }

    fn read_i32(&mut self) -> crate::Result<i32> {
        (**self).read_i32()
    }

}


impl<P> TOutputProtocol for Box<P>
where
    P: TOutputProtocol + ?Sized,
{
    fn write_bytes(&mut self, b: &[u8]) -> crate::Result<()> {
        (**self).write_bytes(b)
    }

    fn write_i32(&mut self, i: i32) -> crate::Result<()> {
        (**self).write_i32(i)
    }

    fn flush(&mut self) -> crate::Result<()> {
        (**self).flush()
    }
}