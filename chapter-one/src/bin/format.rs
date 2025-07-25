use std::fmt::format;

fn main() {
    let colour = "red";

    let favourite = format!("My favourit colour is {}", colour);
    println!("{}", favourite);

    let hello = "hello";
    let world = "world!";
    let hello_world = format!("{}{}", hello, world);

    println!("{}", hello_world);

    // format can concatenate any data types that implement the Display trait
    let favourite_num = format!("My favorite num is {}", 42);
    println!("{}", favourite_num);

    // if you want to include certain parameters multiple times, you can use positional parameters
    let duck_duck_goose = format!("{0}, {0}, {0}, {1}!", "duck", "goose");

    // You can even name your paramters!
    let introduction = format!(
        "My name is {surname}, {firstname} {middlename}",
        surname="Bond",
        firstname="James",
        middlename="W."
    );
    println!("{}", introduction);

}