mod animate;

#[cfg(test)]
mod tests {
    use std::io::{self, BufWriter, Cursor, Read};

    use super::*;

    #[test]
    fn it_works() {
        let mut buffer = Cursor::new(Vec::new());
        {
            let mut buf_writer = BufWriter::new(&mut buffer);
            animate::start(&mut buf_writer);
        }
        buffer.set_position(0);
        let mut content = String::new();
        let _ = buffer.read_to_string(&mut content).expect("Failed to read output");
       assert_eq!(content, "|\0x08\n/\0x08\n-\0x08\n\\\0x08\n"); 

    }
}
