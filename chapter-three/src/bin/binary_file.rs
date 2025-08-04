use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt, BE,
                LE};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read};
use std::io::prelude::*;

fn main() {
    let path = "./bar.bin";
    write_dummy_protocol(path).expect("Failed to read file");
    let payload = read_protocol(path).expect("Failed to read file");

    print!("The protocol contained the following payload: ");
    for num in payload {
        println!("0x:{:X}", num)
    }

    println!();
}

fn write_dummy_protocol(path: &str) -> io::Result<()> {
    let file = File::create(path)?;
    let mut buf_writer = BufWriter::new(file);

    let magic = b"MyProtocol";
    buf_writer.write_all(magic)?;

    let endianness = b"LE";
    buf_writer.write_all(endianness)?;

    buf_writer.write_u32::<LE>(0xDEAD)?;
    buf_writer.write_u32::<LE>(0xBEEF)?;

    Ok(())
}

fn read_protocol(path: &str) -> io::Result<Vec<u32>> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);

    let mut start = [0u8; 10];
    buf_reader.read_exact(&mut start)?;

    if &start != b"MyProtocol" {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Protocol didn't start with the expected magic string",
        ));
    }

    let mut endian = [0u8; 2];
    buf_reader.read_exact(&mut endian)?;

    match &endian {
        b"LE" => read_protocoll_payload::<LE, _>(&mut buf_reader),
        b"BE" => read_protocoll_payload::<BE, _>(&mut buf_reader),
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to parse endianess",
        )),
    }
}

fn read_protocoll_payload<E, R>(reader: &mut R) -> io::Result<Vec<u32>>
where
    E: ByteOrder,
    R: ReadBytesExt,
{
    let mut payload = Vec::new();
    const SIZE_OF_U32: usize = 4;

    loop {
        let mut raw_payload = [0; SIZE_OF_U32];

        match reader.read(&mut raw_payload)? {
            0 => return Ok(payload),
            SIZE_OF_U32 => {
                let as_u32 = raw_payload.as_ref().read_u32::<E>()?;
                payload.push(as_u32);
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                        "Payload ended unexpectedly",
                ))
            }
        }
    }

}