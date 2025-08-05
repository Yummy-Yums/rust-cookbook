use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::fs::{File, OpenOptions};
use serde::*;

#[derive(Serialize, Deserialize)]
struct Planet {
    name: String,
    radius: f32,
    distance_from_sun: f32,
    gravity: f32
}

fn main() {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("solar_system_compared_to_earth.csv")
        .expect("failed to create csv file");

    let buf_writer = BufWriter::new(&file);
    write_records(buf_writer).expect("Failed to write csv");

    let mut buf_reader = BufReader::new(&file);
    buf_reader
        .seek(SeekFrom::Start(0))
        .expect("Failed to jump to the beginning of the csv file");

    read_records(buf_reader).expect("Failed to read csv");
}

fn read_records<R>(reader: R) -> csv::Result<()>
where R: Read
{
    let mut rdr = csv::Reader::from_reader(reader);

    for result in rdr.deserialize() {
        println!("-------");
        let planet: Planet = result?;

        println!("Name: {}", planet.name);
        println!("Radius: {}", planet.radius);
        println!("Distance from sun: {}", planet.distance_from_sun);
        println!("Surface gravity: {}", planet.gravity);
    }

    Ok(())
}

fn write_records<W>(writer: W) -> csv::Result<()>
where
    W: Write
{
    let mut wtr = csv::Writer::from_writer(writer);

    wtr.serialize(Planet {
        name: "Mercury".to_string(),
        radius: 0.38,
        distance_from_sun: 0.47,
        gravity: 0.38
    })?;

    wtr.serialize(Planet {
        name: "Venus".to_string(),
        radius: 0.95,
        distance_from_sun: 0.73,
        gravity: 0.9
    })?;

    wtr.serialize(Planet {
        name: "Earth".to_string(),
        radius: 1.0,
        distance_from_sun: 1.0,
        gravity: 1.0
    })?;

    wtr.serialize(Planet {
        name: "Mars".to_string(),
        radius: 0.53,
        distance_from_sun: 1.67,
        gravity: 0.38
    })?;

    wtr.serialize(Planet {
        name: "Jupiter".to_string(),
        radius: 11.21,
        distance_from_sun: 5.46,
        gravity: 2.53
    })?;

    wtr.serialize(Planet {
        name: "Saturn".to_string(),
        radius: 11.21,
        distance_from_sun: 5.46,
        gravity: 2.53
    })?;

    wtr.serialize(Planet {
        name: "Uranus".to_string(),
        radius: 11.21,
        distance_from_sun: 5.46,
        gravity: 2.53
    })?;

    wtr.serialize(Planet {
        name: "Neptune".to_string(),
        radius: 11.21,
        distance_from_sun: 5.46,
        gravity: 2.53
    })?;

    wtr.flush()?;
    Ok(())
}