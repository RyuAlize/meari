use std::io;
use flume::{Sender, Receiver};

use crate::screen::ScreenCap;
use crate::util;

pub struct RemoteDesktopService {
    screen_cap: ScreenCap,

    signal_rx: Receiver<()>,
}

impl RemoteDesktopService {
    pub fn new(signal_rx: Receiver<()>) -> io::Result<Self> {
        Ok(Self { 
            screen_cap: ScreenCap::new()?,
            signal_rx,
        })
    }

    pub fn run() -> io::Result<()>{
        let mut sc=  ScreenCap::new()?;
        loop {
            let data = sc.capture()?;
            let compressed_data = util::compress(data)?;
            
        }
    }
}