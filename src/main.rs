mod screen;
//mod server;
mod config;
mod util;

use flume::Receiver;
use scrap::{Capturer, Display, Frame};
use image::codecs::png;
use image::{ImageEncoder, ColorType};
use std::fs::OpenOptions;
use std::thread;
use std::io::Result;


use screen::Screen;
fn main() -> Result<()>{
    let (mut sc, mut rx) = Screen::new()?;
    
    Ok(())

        
        // Save the image.
  /*   let img = OpenOptions::new()
        .write(true)
        .create(true)
        .open("test.png").unwrap();
    let png_file = png::PngEncoder::new(img);
    png_file.write_image(bitflipped.as_ref(), w as u32, h as u32, ColorType::Rgba8);
         */
   
}
