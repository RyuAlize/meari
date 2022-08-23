mod connection;
mod codec;
mod config;
mod error;
mod mirror;
mod screen;
mod server;
mod protocol;
mod util;

use std::io::Result;
use screen::ScreenCap;
#[tokio::main]
async fn main() -> Result<()>{
/*     let (mut sc, mut rx) = ScreenCap::new()?;

    task::spawn( async move {
        sc.frame().await;
    }); */
    
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
