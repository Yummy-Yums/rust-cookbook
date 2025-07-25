use std::borrow::Cow;

fn main() {

}

struct NameLength {
    name: String,
    length: usize,
}

impl NameLength {
    fn new(name: &str) -> Self {
        NameLength {
            length: name.len(),
            name: name.to_string(),
        }
    }

    fn print(&self) {
        println!(
            "The name '{}' is '{}' characters long",
            self.name,
            self.length
        )
    }
}


struct NameLengthCow<'a> {
    name: Cow<'a, str>,
    length: usize,
}

impl<'a> NameLengthCow<'a> {
    fn new<S>(name: S) -> Self
    where
      S: Into<Cow<'a, str>>,
    {
        let name: Cow<'a, str> = name.into();
        NameLengthCow {
            length: name.len(),
            name ,
        }
    }

    fn print(&self) {
        println!(
            "The name '{}' is '{}' characters long",
            self.name,
            self.length
        )
    }
}