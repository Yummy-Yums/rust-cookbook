
fn main() {

    let fruits = vec!["apple", "tomato", "pear"];

    println!("fruits: {:?}", fruits);

    let mut fruits = Vec::new();

    fruits.push("apple");
    fruits.push("tomato");
    fruits.push("pear");

    println!("fruits: {:?}", fruits);

    // Remove the last element
    let last = fruits.pop();
    if let Some(last) = last {
        println!("Removed {} from {:?}", last, fruits);
    }

    fruits.insert(1, "apple");
    println!("fruits after insertion: {:?}", fruits);

    fruits.swap(0, 1);
    println!("fruits after swap: {:?}", fruits);


    // Access the first and last elements
    let first = fruits.first();
    if let Some(first) = first {
        println!("First fruit: {}", first)
    }

    let last = fruits.last();
    if let Some(last) = last {
        println!("Last fruit: {}", last)
    }

    // Access arbitrary elements
    let second = fruits.get(1);
    if let Some(second) = second {
        println!("Last fruit: {}", second)
    }

    let second = fruits[1];
    println!("Second fruit: {}", second);

    let bunch_of_zeroes = vec![0; 5];
    println!("bunch_of_zeroes: {:?}", bunch_of_zeroes);

    // Remove some items and shift all that come after into place
    let mut nums = vec![1, 2, 3, 4];
    let second_num = nums.remove(1);
    println!("Removed {} from {:?}", second_num, nums);

    // Filter the vector in place
    let mut names = vec!["Aaron", "Felicia", "Alex", "Danniel"];
    // names.retain(|name|)



}