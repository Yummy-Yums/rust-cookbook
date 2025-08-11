use std::{error, fmt, io, num, result};
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug)]
enum AgeReaderError {
    Io(io::Error),
    Parse(num::ParseIntError),
    NegativeAge(),
}

type Result<T> = result::Result<T, AgeReaderError>;

impl error::Error for AgeReaderError {
    fn description(&self) -> &str {
        match *self {
            AgeReaderError::Io(ref err) => err.description(),
            AgeReaderError::Parse(ref err) => err.description(),
            AgeReaderError::NegativeAge() => "Age is negative",

        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            AgeReaderError::Io(ref err) => Some(err),
            AgeReaderError::Parse(ref err) => Some(err),
            AgeReaderError::NegativeAge() => None,
        }
    }
}

impl fmt::Display for AgeReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
           AgeReaderError::Io(ref err) => write!(f, "IO error: {}", err),
           AgeReaderError::Parse(ref err) => write!(f, "Parse error: {}", err),
           AgeReaderError::NegativeAge() => write!(f, "Age is negative"),
        }
    }
}

impl From<io::Error> for AgeReaderError {
    fn from(err: io::Error) -> AgeReaderError {
        AgeReaderError::Io(err)
    }
}

impl From<num::ParseIntError> for AgeReaderError {
    fn from(err: num::ParseIntError) -> AgeReaderError {
        AgeReaderError::Parse(err)
    }
}

fn main(){
    const FILENAME: &str = "age.txt";
    let result = read_age(FILENAME);

    match result {
        Ok(num) => println!("{} contains the age {}", FILENAME, num),
        Err(AgeReaderError::Io(err)) => eprintln!(
            "Failed to read the contents of {} as a number: {}",
            FILENAME, err
        ),
        Err(AgeReaderError::Parse(err)) => eprintln!(
            "Failed to parse the contents of {} as a number: {}",
            FILENAME, err
        ),
        Err(AgeReaderError::NegativeAge()) => eprintln!(
            "The age in the file is negative"
        )
    }

    fn read_age(file_name: &str) -> Result<i32> {
        let file = File::open(file_name)?;
        let mut buf_reader = BufReader::new(file);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content)?;
        let age: i32 = content.trim().parse()?;

        if age.is_positive(){
            Ok(age)
        } else {
            Err(AgeReaderError::NegativeAge())
        }
    }
}