use std::ops::MulAssign;
use std::fmt::Display;

#[derive(Debug)]
struct DoubleVec<T>(Vec<T>);

impl<T> From<Vec<T>> for DoubleVec<T>
where
    T: MulAssign<i32>,
{
    fn from(mut vec: Vec<T>) -> Self {
        for elem in &mut vec {
            *elem *= 2;
        }

        DoubleVec(vec)
    }

}

impl<'a, T> From<&'a [T]> for DoubleVec<T>
where
    T: MulAssign<i32> + Clone,
{
    fn from(slice: &'a [T]) -> Self {
        slice.to_vec().into()
    }
}

impl<T> AsRef<Vec<T>> for DoubleVec<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.0
    }
}

fn main() {
    let hello_world = "Hello World".to_string();
    let hello_world: String = "Hello World".into();
    let hello_world = String::from("Hello World!");

    let hello_world_bytes: Vec<u8> = "Hello World!".into();
    let hello_world_bytes = Vec::<u8>::from("Hello World!");

    let vec = vec![1, 2, 3];
    let double_vec = DoubleVec::from(vec);
    println!("Creating a DoubleVec from a Vec: {:?}", double_vec);

    let vec = vec![1, 2, 3];
    let double_vec: DoubleVec<_> = vec.into();

    print_elements(double_vec.as_ref());

    easy_public_fun(Some(1337), Some(123), None);
    ergonomic_public_fun(1337, 123, None)
}

fn print_elements<T>(slice: &[T])
where
    T: Display,
{
    for elem in slice {
        println!("{}", elem);
    }
    println!();
}

fn easy_public_fun(foo: Option<i32>, bar: Option<i32>, baz: Option<i32>) {
    println!(
        "easy_public_func = foo: {:?}, bar: {:?}, baz: {:?}",
        foo, bar, baz
    );
}

fn ergonomic_public_fun<Foo, Bar, Baz>(foo: Foo, bar: Bar, baz: Baz)
where
    Foo: Into<Option<i32>>,
    Bar: Into<Option<i32>>,
    Baz: Into<Option<i32>>,
{
    let foo: Option<i32> = foo.into();
    let bar: Option<i32> = bar.into();
    let baz: Option<i32> = baz.into();

    println!(
        "ergonomic_public_fun = foo: {:?}, bar: {:?}, baz: {:?}",
        foo, bar, baz
    )
}