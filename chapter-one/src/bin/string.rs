fn main() {
    let mut s = String::new();

    s.push('H');
    s.push('i');

    println!("s: {}", s);

    let s =  String::new();

    println!("s: {}", s);

     for s in "I'm2fast4you".split(char::is_numeric){
         println!("{}", s);
     }

     let some_numbers: Vec<_> = (1..4).cycle().take(10).collect();
     println!("some_numbers: {:?}", some_numbers);

     let some_numbers: Vec<_> = (1..4).chain(10..14).collect();
     println!("some_numbers: {:?}", some_numbers);
}