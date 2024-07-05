use std::io::{self, BufWriter, Write};
use std::thread::sleep;
use std::time::Duration;
use std::{time, thread};

const ROLLING  : [char;4] = ['|', '/', '-', '\\'];

pub fn start<W:Write>(handle: &mut BufWriter<W>) -> io::Result<()> {
    for frame in ROLLING.into_iter(){
        write!(handle, "{}", frame).expect("Failed to print to std");
        handle.flush()?;
        sleep(Duration::from_secs(1)); 
        write!(handle, "\x08").expect("Failed to print to std");
        handle.flush()?;
    }

    Ok(())
}
