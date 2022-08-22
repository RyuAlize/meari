use std::fs::OpenOptions;
use std::io::Result;
use std::thread;
use std::io::ErrorKind::WouldBlock;

use scrap::{Capturer, Display, Frame};
use image::codecs::png;
use image::{ImageEncoder, ColorType};
use flume::{Sender, Receiver};
use tokio::task;

use crate::config::FPS;
pub struct Screen {
    capturer: Capturer,
    pixel_frame_tx: Sender<Vec<u8>>
}

impl Screen {
    pub fn new() -> Result<(Self, Receiver<Vec<u8>>)> {
        let display = Display::primary()?;
        let mut capturer = Capturer::new(display)?;
        let (tx, rx) = flume::unbounded();
        Ok((Self { capturer, pixel_frame_tx: tx }, rx))
    }


    pub async fn frame(&mut self) {
        let (w, h) = (self.capturer.width(), self.capturer.height());
        let one_second = std::time::Duration::new(1, 0);
        let one_frame = one_second / FPS;
        loop{
            match self.capturer.frame() {
                Ok(buffer) => {
                    match self.pixel_frame_tx.send(frame_to_bytes(buffer, w, h)) {
                        Ok(_) =>(),
                        Err(_) => break,
                    }
                },
                Err(error) => {
                    if error.kind() == WouldBlock {
                        thread::sleep(one_frame);
                        continue;
                    } else {
                        break;
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