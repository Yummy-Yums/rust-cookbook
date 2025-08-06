use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let mut key_value_map = HashMap::new();
    let stdin = io::stdin();

    println!("Enter a key and a value");

    for input in stdin.lock().lines() {
        let input = input.expect("failed to read line");
        let key_value: Vec<&str> = input.split_whitespace().collect();
        let key = key_value[0].to_string();
        let value = key_value[1].to_string();

        println!("Saving key-value pair: {} -> {}", key, value);

        key_value_map.insert(key, value);
        println!(
            "Enter another pair or stop by pressing '{}'",
            END_OF_TRANSMISSION
        )
    }

    let json = serde_json::to_string(&key_value_map)
        .expect("failed to convert HashMap into JSON");

    println!("Your input has been made into the following JSON:");
    println!("{}", json);

}

#[cfg(target_os = "windows")]
const END_OF_TRANSMISSION: &str = "Ctrl Z";

#[cfg(not(target_os = "windows"))]
const END_OF_TRANSMISSION: &str = "Ctrl D";