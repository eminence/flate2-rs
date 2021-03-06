extern crate flate2;

use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::write::{GzEncoder, GzDecoder};

// Compress a sample string and print it after transformation.
fn main() {
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write(b"Hello World").unwrap();
    let bytes = e.finish().unwrap();
    println!("{}", decode_writer(bytes).unwrap());
}

// Uncompresses a Gz Encoded vector of bytes and returns a string or error
// Here &[u8] implements Read
fn decode_writer(bytes: Vec<u8>) -> io::Result<String> {
    let mut writer = Vec::new();
    let mut decoder = GzDecoder::new(writer);
    decoder.write(&bytes[..])?;
    decoder.try_finish()?;
    writer = decoder.finish()?;
    let return_string = String::from_utf8(writer).expect("String parsing error");
    Ok(return_string)
}
