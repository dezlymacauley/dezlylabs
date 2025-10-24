// This adds the `Debug` trait to the struct so that it can be printed
// in a human readable way by the `println!()` macro, using the format 
// specifier `:?` or `:#?`
#[derive(Debug)]
struct Node {
    value: i32,
    // `Box<Node>` is a Heap-Allocated Pointer.
    // In simple terms, it is pointer to a `Node` struct 
    // that is stored in Heap memory.

    // I placed this inside the Option enum to show that 
    // the field called `next_node` could either be of the variable type 
    // `Some(Node), or `None`

    // None is the closet thing that Rust has to null.
    next_node: Option<Box<Node>>,
}

// This is an implementation block.
// All the functions inside this block are not stand-alone functions,
// but methods that can be called by any variable 
// that is an instance of the Node class.
impl Node {

    // This method called `new` creates a new Node
    fn new(value: i32) -> Self {
        Node {
            value: value,
            next_node: None
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
}
