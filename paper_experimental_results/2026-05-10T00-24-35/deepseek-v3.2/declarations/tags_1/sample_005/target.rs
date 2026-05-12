struct Node {
    count: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn main() {
    let left_child = Node {
        count: 2,
        left: None,
        right: None,
    };
    
    let right_child = Node {
        count: 3,
        left: None,
        right: None,
    };
    
    let s = Node {
        count: 1,
        left: Some(Box::new(left_child)),
        right: Some(Box::new(right_child)),
    };
    
    let sp = &s;
    
    let result = if sp.left.as_ref().map(|n| n.count) != Some(2) {
        1
    } else if sp.right.as_ref().map(|n| n.count) != Some(3) {
        2
    } else if sp.left.as_ref().map(|n| n as *const _) == sp.right.as_ref().map(|n| n as *const _) {
        3
    } else {
        0
    };
    
    println!("{}", result);
}