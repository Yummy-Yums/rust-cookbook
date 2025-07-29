use std::env;

fn main() {
    println!("Listing all env vars:");

    for (key, val) in env::vars(){
        println!("{}: {}", key, val);
    }

    let key = "PORT";
    println!("Setting env var {}", key);

    unsafe { env::set_var(key, "8080"); }

    print_env_var(key);

    println!("Removing ");
    unsafe { env::remove_var(key); }

    print_env_var(key);

}

fn print_env_var(key: &str){
    match env::var(key){
        Ok(val ) => println!("{}: {}", key, val),
        Err(e) => println!("Couldn't print env var {}: {}", key, e)
    }
}