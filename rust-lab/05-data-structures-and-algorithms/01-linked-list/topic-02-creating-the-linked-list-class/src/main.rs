#[derive(Debug)]
struct Node {
    value: i32,
    next_node: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value: value,
            next_node: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList {
    // `head` is of the type `Option<Box<Node>>`
    // This means that `head` head could be either:
    // 1. Some(Box<Node>), which is a Heap-Allocated pointer to a Node
    // `Box<Node>` means that the Node is stored in Heap memory.
    // 2. None, which will happen if the list is empty.
    head: Option<Box<Node>>,

    // `tail` is of the type `Option<Box<Node>>`
    // This means that `tail` head could be either:
    // 1. Some(Box<Node>), which is a Heap-Allocated pointer to a Node
    // `Box<Node>` means that the Node is stored in Heap memory.
    // 2. None, which will happen if the list is empty.
    tail: Option<Box<Node>>,

    // This will keep track of the number of Nodes in the list.
    // `usize` is ideal because it only stores negative numbers and the number
    // of Nodes in the LinkedList can never be negative.

    // The largest number that usize can store is platform-specific.
    // To check what this number is on your system

    // you can run this:
    // println!("usize max: {}", usize::MAX);
    // On my system its: 18446744073709551615
    // That's eighteen quintillion,
    // four hundred forty-six quadrillion,
    // seven hundred forty-four trillion,
    // seventy-three billion,
    // seven hundred nine million,
    // five hundred fifty-one thousand,
    // six hundred fifteen.”

    // It is very unlikely that you Linked List will ever hit that number.
    length: usize,
}

impl LinkedList {
    fn new(value: i32) -> Self {

        LinkedList {
            head: 
            tail: 
            length: 1,
        }

    }
}

fn main() {
    // `Box<Node>` is a Heap-Allocated Pointer, that points to an instance
    // of the Node class that is stored in Heap Heap memory.
    let node_one: Box<Node> = Box::new(Node::new(56));

    // This is compact debug format.
    // The struct will be printed in a single line.
    println!("node_one - compact debug format: {node_one:?}");
    println!();
    // node_one - compact debug format: Node { value: 56, next: None }

    // This is the pretty-print format.
    // The struct will be printed in a more human-readable way using
    // multiple lines.
    println!("node_one - pretty-print format");
    println!("{node_one:#?}");
    println!();

    // node_one - pretty-print format
    // Node {
    //     value: 56,
    //     next: None,
    // }

    //_________________________________________________________________________

    // NOTE: `println!()` will automatically dereference the Heap-Allocated
    // pointer for you
    println!("value field of node_one: {}", node_one.value);
    println!("next field of node_one: {:?}", node_one.next_node);
    println!();
    // value field of node_one: 56
    // next field of node_one: None

    //_________________________________________________________________________
    // You want to be explicit about the dereferencing

    println!("value field of node_one: {}", (*node_one).value);
    println!("next field of node_one: {:?}", (*node_one).next_node);

    //_________________________________________________________________________
    println!("usize max: {}", usize::MAX);
}
