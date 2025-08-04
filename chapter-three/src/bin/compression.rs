use std::io::{self, SeekFrom};
use std::io::prelude::*;
use flate2::{Compression, FlateReadExt};
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read};
use std::path::Path;

fn main() {

    let bytes = b"I have a dream that one day this nation will rise up,\
    and live out the true meaning of its creed";

    println!("Original: {:?}", bytes.as_ref());

    let encoded = encode_bytes(bytes.as_ref()).expect("Failed to encode bytes");
    println!("Encoded: {:?}", encoded);

    let decoded = decode_bytes(&encoded).expect("Failed to encode bytes");
    println!("Decoded: {:?}", decoded);
    let path = Path::new("../ferris.png");
    println!("Writing: {:?}", path);
    let original = File::open(path).expect("Failed to open file");
    let mut original_reader = BufReader::new(original);

    let data = encode_file(&mut original_reader).expect("Failed to encode file");

    let encoded = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("ferris_encoded.zlib")
        .expect("Failed to encode file");

    let mut encoded_reader = BufReader::new(&encoded);
    let mut encoded_writer = BufWriter::new(&encoded);

    encoded_writer
        .write_all(&data)
        .expect("Failed to reset file");

    encoded_reader
        .seek(SeekFrom::Start(0))
        .expect("Failed to seek to start of encoded file");

    let data = decode_file(&mut encoded_reader).expect("Failed to decode file");

    let mut decoded =
        File::create("ferris_decoded.png").expect("Failed to create decoded file");

    decoded
        .write_all(&data)
        .expect("Failed to write decoded file");

}

fn encode_bytes(bytes: &[u8]) -> io::Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::Default);
    encoder.write_all(bytes)?;
    encoder.finish()
}

fn decode_bytes(bytes: &[u8]) -> io::Result<Vec<u8>> {
    let mut encoder = ZlibDecoder::new(bytes);
    let mut buffer = Vec::new();
    encoder.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn encode_file(file: &mut dyn Read) -> io::Result<Vec<u8>> {
    let mut encoded = file.zlib_encode(Compression::Best);
    let mut buffer = Vec::new();
    encoded.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn decode_file(file: &mut dyn Read) -> io::Result<Vec<u8>> {
    let mut buffer = Vec::new();
    file.zlib_decode().read_to_end(&mut buffer)?;
    Ok(buffer)
}


