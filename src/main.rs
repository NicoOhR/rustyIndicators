
use std::io::{self, BufWriter};
use loading_icon::animate::Context;

pub fn main() {
    
    let mut handle = BufWriter::new(io::stdout()); 
    let mut context : Context = Context::new();
    let _ = loading_icon::animate::start(&mut handle, &mut context);
    loading_icon::animate::stop(&mut context);
}
