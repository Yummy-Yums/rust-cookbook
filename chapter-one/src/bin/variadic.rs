macro_rules! multiply {

    ( $last:expr ) => { $last };

    ( $head:expr, $( $rest:expr ),* ) => {
        $head * multiply!($( $rest ),+)
    }

}

fn main() {

    let val = multiply!(2, 4, 8);
    println!("2 * 4 * 8 = {}", val);

}