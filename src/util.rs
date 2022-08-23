use std::future::Future;
use std::pin::Pin;
use std::io::{self, prelude::*};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::GzDecoder;

pub type BoxFuture<T, E> = Pin<Box<dyn Future<Output = std::result::Result<T, E>> + Send>>;


pub fn compress(bytes: Vec<u8>) -> io::Result<Vec<u8>> {
    let mut e = ZlibEncoder::new(bytes, Compression::default());
    e.finish()
} 

pub fn decompress(bytes: &[u8]) -> io::Result<Vec<u8>> {
    let mut gz = GzDecoder::new(bytes);
    let mut buf = vec![];
    gz.read_to_end(&mut buf)?;
    Ok(buf)
}

