use std::sync::{Arc, Mutex};

pub struct Node {
    pub data:String,
}

pub fn ask_data(shared: &Arc<Mutex<Vec<Box<Node>>>>) -> bool{
    let mut input = String::new();

    println!("Enter data (quit to exit):");
    std::io::stdin().read_line(&mut input).unwrap();

    let trimmed = input.trim().to_string();

    if trimmed == "quit" {
        return true;
    }

    let node = Box::new(Node { data: trimmed });

    let mut vec = shared.lock().unwrap();
    vec.push(node);

    false
}

pub fn allocate_node(data: Node) -> Box<Node>{
    let a= data;
    Box::new(a)
}


