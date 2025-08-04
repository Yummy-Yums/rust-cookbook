use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Lines, SeekFrom, Write};
use std::io::prelude::*;

fn main() {

    let path = "./foo.txt";
    println!("Writing some data to '{}'", path);

    write_file(path, "Hello World! \n").expect("Failed to write to file");

    let content = read_file(path).expect("Failed to read from file");
    println!("The file '{}' contains:", path);
    println!("{}", content);

    println!("Writing new data to '{}'", path);

    write_file(path, "New content\n").expect("Failed to write to file");

    let content = read_file(path).expect("Failed to read from file");
    println!("The file '{}' now contains:", path);
    println!("{}", content);

    println!("Appending data to '{}'", path);
    append_file(path, "Some more content\n").expect("Failed to append to file");

    println!("The file '{}' now contains:", path);

    let lines = read_file_iterator(path).expect("Failed to read file");

    for line in lines {
        println!("{}", line.expect("Failed to read line"));
    }

    append_and_read(path, "Last line in the file, goodbye").expect("Failed to read file");
}

fn read_file(path: &str) -> io::Result<String> {
    let file = File::open(path)?;

    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

fn read_file_iterator(path: &str) -> io::Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    Ok(buf_reader.lines())
}

fn write_file(path: &str, content: &str) -> io::Result<()> {
    let file = File::create(path)?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(content.as_bytes())?;
    Ok(())
}

fn append_file(path: &str, content: &str) -> io::Result<()> {
    let file = OpenOptions::new().append(true).open(path)?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(content.as_bytes())?;
    Ok(())
}

fn append_and_read(path: &str, content: &str) -> io::Result<()> {
    let file = OpenOptions::new().read(true).append(true).open(path)?;

    let mut buf_reader= BufReader::new(&file);
    let mut buf_writer = BufWriter::new(&file);

    let mut file_content = String::new();
    buf_reader.read_to_string(&mut file_content)?;
    println!("File before appending:\n{}", file_content);

    let pos = buf_reader.seek(SeekFrom::Current(0))?;
    buf_writer.write_all(content.as_bytes())?;
    buf_writer.flush()?;
    buf_reader.seek(SeekFrom::Start(pos))?;

    buf_reader.read_to_string(&mut file_content)?;
    println!("File after appending:\n{}", file_content);

    Ok(())
}