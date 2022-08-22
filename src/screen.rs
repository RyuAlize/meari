use std::fs::OpenOptions;
use std::io::Result;
use std::thread;
use std::io::ErrorKind::WouldBlock;
use std::future::Future;

use scrap::{Capturer, Display, Frame};
use image::codecs::png;
use image::{ImageEncoder, ColorType};
use flume::{Sender, Receiver};
use tokio::task;
use tokio::sync::{broadcast, mpsc, Semaphore};
use crate::config::FPS;
pub struct ScreenCap {
    capturer: Capturer,
    signal_rx: Receiver<Semaphore>,
    pixel_frame_tx: Sender<Vec<u8>>
}

impl Future for ScreenCap {
    type Output = ();
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        todo!()
    }
}
impl ScreenCap {
    pub fn new(signal_rx: Receiver<Semaphore>) -> Result<(Self, Receiver<Vec<u8>>)> {
        let display = Display::primary()?;
        let mut capturer = Capturer::new(display)?;
        let (tx, rx) = flume::unbounded();
        Ok((Self { capturer, signal_rx, pixel_frame_tx: tx }, rx))
    }


    pub async fn capture(&mut self) -> Result<Vec<u8>> {
        let (width, height) = (self.capturer.width(), self.capturer.height());
        let one_second = std::time::Duration::new(1, 0);
        let one_frame = one_second / FPS;
        loop{
            match self.capturer.frame() {
                Ok(buffer) => {
                    return Ok(frame_to_bytes(buffer, width, height));
                },
                Err(error) => {
                    if error.kind() == WouldBlock {
                        thread::sleep(one_frame);
                        continue;
                    } else {
                        return Err(error);
                    }
                }
            };        
        } 
    } 

}

fn frame_to_bytes(buffer: Frame, width: usize, height: usize)-> Vec<u8> {
    let mut bitflipped = Vec::with_capacity(width * height * 3);
        let stride = buffer.len() / height;
        for y in 0..height {
            for x in 0..width {
                let i = stride * y + 3 * x;
                bitflipped.extend_from_slice(&[
                    buffer[i + 2],
                    buffer[i + 1],
                    buffer[i],
                ]);
            }
        }
        bitflipped
}