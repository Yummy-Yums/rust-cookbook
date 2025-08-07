use std::fs::File;
use std::io::BufReader;
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
}

fn main() {}