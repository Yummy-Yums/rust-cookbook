use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, Read};
use std::result::Result;


#[derive(Debug)]
struct Node<T>{
    data: T,
    child_nodes: Option<(BoxedNode<T>, BoxedNode<T>)>,
}

type BoxedNode<T> = Box<Node<T>>;

impl<T> Node<T>{
    fn new(data: T) -> Self {
        Node {
            data,
            child_nodes: None
        }
    }

    fn is_leaf(&self) -> bool {
        self.child_nodes.is_none()
    }

    fn add_child_nodes(&mut self, a: Node<T>, b: Node<T>){
        assert!(
            self.is_leaf(),
            "Tried to add child_nodes to a node that is not a leaf"
        );
        self.child_nodes = Some((Box::new(a), Box::new(b)));
    }
}

trait Animal: Debug {
    fn sound(&self) -> &'static str;
}

#[derive(Debug)]
struct Dog;

impl Animal for Dog {
    fn sound(&self) -> &'static str {
        "Woof!"
    }
}

#[derive(Debug)]
struct Cat;
impl Animal for Cat {
    fn sound(&self) -> &'static str {
        "Meow!"
    }
}

fn main() {
    let mut root = Node::new(12);
    root.add_child_nodes(Node::new(3), Node::new(5));
    root.child_nodes
        .as_mut()
        .unwrap()
        .0
        .add_child_nodes(Node::new(4), Node::new(6));

    println!("Our binary tree looks this: {:?}", root);

    let mut zoo: Vec<Box<dyn Animal>> = Vec::new();
    zoo.push(Box::new(Dog {}));
    zoo.push(Box::new(Cat {}));

    for animal in zoo {
        println!("{:?} says {}", animal, animal.sound());
    }

    for word in caps_word_iter("do you feel lucky, punk?"){
        println!("{}", word);
    }

    let num = read_file_as_number("number.txt").expect("Failed to read the file as a number");
    println!("number.txt contains the number {}", num);

    let multiplier = create_multiplier(23);
    let result = multiplier(3);
    println!("23 * 3 = {}", result);
}

fn caps_word_iter<'a>(text: &'a str) -> Box<dyn Iterator<Item=String> + 'a>{
    Box::new(text.trim().split(' ').map(|word| word.to_uppercase()))
}

fn read_file_as_number(filename: &str) -> Result<i32, Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut buff_reader = BufReader::new(file);
    let mut content = String::new();
    buff_reader.read_to_string(&mut content)?;
    let number: i32 = content.parse()?;
    Ok(number)
}

fn create_multiplier(a: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |b| a * b)
}