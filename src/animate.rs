use std::io::{BufWriter, Write};

const ROLLING  : [char;4] = ['|', '/', '-', '\\'];

pub fn start<W:Write>(handle: &mut BufWriter<W>) {
    for frame in ROLLING.into_iter(){
        writeln!(handle, "{}\0x08", frame).expect("Failed to print to std");
    }
}
