extern crate rand;

use rand::Rng;
use rand_chacha::{ChaCha20Rng, rand_core::SeedableRng};

fn main() {

    let random_num1 = rand::random::<i32>();
    println!("random_num1: {}", random_num1);

    let random_num2: i32 = rand::random();
    println!("random_num2: {}", random_num2);

    let random_char = rand::random::<char>();
    println!("random_char: {}", random_char);

    use rand::Rng;

    let mut rng = rand::rng();

    if rng.random(){
        println!("This message has a 50-50 chance of being printed")
    }

    let random_num3 = rng.random_range(0..10);
    println!("random_float: {}", random_num3);

    let random_float = rng.random_range(0.0..100.0);
    println!("random_float: {}", random_float);

    let mut chacha_rng = ChaCha20Rng::from_seed(Default::default());
    let random_chacha_num = chacha_rng.random::<i32>();

    println!("random_chacha_num: {}", random_chacha_num)


}