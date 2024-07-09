use std::io::{self, BufWriter, Write};
use std::thread::sleep;
use std::time::Duration;
use std::{time, thread};

const ROLLING  : [char;4] = ['|', '/', '-', '\\'];

pub struct Context {
    active : bool
}

impl Context {
    pub fn new() -> Self {
        Context{active : false}
    }
    fn stop(&mut self)  {
       self.active = false; 
    }
    fn start(&mut self) {
        self.active = true;
    }
}


pub fn start<W:Write>(handle: &mut BufWriter<W>, context : &mut Context) -> io::Result<()> {
    context.start(); 
    if context.active { loop {
        for frame in ROLLING.into_iter(){
            write!(handle, "{}", frame)?;
            handle.flush()?;
            sleep(Duration::from_millis(250)); 
            write!(handle, "\x08")?;
            handle.flush()?;
        }
    }}
    Ok(())
}

pub fn stop(context : &mut Context) {
    context.stop();
}
