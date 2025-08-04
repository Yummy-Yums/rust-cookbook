use std::io::{Cursor, Seek, SeekFrom};
use byteorder::{BigEndian, LittleEndian, WriteBytesExt, ReadBytesExt};

fn main() {

    let binary_nums = vec![2, 3, 12, 8, 5, 0];

    let mut buff = Cursor::new(binary_nums);

    let first_byte = buff.read_u8().expect("Failed to read first byte as int");
    println!("First byte: {:b}", first_byte);

    let second_byte_as_int = buff.read_i8().expect("Failed to read first byte as int");
    println!("First byte: {:b}", second_byte_as_int);

    println!("Before: {:?}", buff);
    buff.write_u8(123).expect("Failed to overwrite a byte");
    println!("After: {:?}", buff);

    println!("Old position: {}", buff.position());
    buff.set_position(0);
    println!("New position: {}", buff.position());

    buff.seek(SeekFrom::End(0)).expect("Failed to seek end");
    println!("Last position: {}", buff.position());

    buff.set_position(0);
    let as_u32 = buff.read_u32::<LittleEndian>()
        .expect("Failed to read bytes");

    println!(
        "First four bytes as u32 in little endian order:\t{}",
        as_u32
    );

    buff.set_position(0);
    let as_u32 = buff.read_u32::<BigEndian>()
        .expect("Failed to read bytes");

    println!(
        "First four bytes as u32 in big endian order:\t{}",
        as_u32
    );

    println!("Before appending: {:?}", buff);
    buff.seek(SeekFrom::End(0)).expect("Failed to seek end");
    buff.write_f32::<LittleEndian>(-33.4)
        .expect("Failed to write to bytes");
    println!("After appending: {:?}", buff);

    let mut read_buffer = [0; 5];
    buff.set_position(0);
    buff.read_u16_into::<LittleEndian>(&mut read_buffer)
        .expect("Failed to read bytes");

    println!(
        "All byte as u16s in little endian order: {:?}",
        read_buffer
    )



}