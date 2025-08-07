use std::collections::HashMap;
use std::sync::RwLock;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref CURRENCIES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("EUR", "Euro");
        m.insert("USD", "U.S. Dollar");
        m.insert("CHF", "Swiss Francs");
        m
    };
}

lazy_static! {
    static ref CLIENTS: RwLock<Vec<String>> = RwLock::new(Vec::new());
}

fn extract_day(date: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(\d{2})-(\d{2})-(\d{4})")
                    .expect("Failed to create regex");
    }

    RE.captures(date)
        .and_then(|cap| cap.get(1).map(|day| day.as_str()))
}


fn main() {

    let usd = CURRENCIES.get("USD");
    if let Some(usd) = usd {
        println!("USD stands for {}", usd);
    }

    if let Some(chf) = CURRENCIES.get("CHF") {
        println!("CHF stands for {}", chf);
    }

    CLIENTS
        .write()
        .expect("Failed to unlock clients for writing")
        .push("192.168.0.1".to_string());

    let clients = CLIENTS
        .read()
        .expect("Failed to unlock clients for reading");

    let first_client = clients.get(0).expect("CLIENTS is empty");
    println!("The first client is: {}", first_client);

    let date = "12.01.2018";
    if let Some(day) = extract_day(date){
        println!("The date \"{}\" contains the day \"{}\"", date, day)
    }
}


