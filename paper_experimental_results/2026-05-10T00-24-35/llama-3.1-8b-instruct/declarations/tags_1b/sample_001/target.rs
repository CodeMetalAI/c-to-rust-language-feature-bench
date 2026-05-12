struct Node {
    count: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn main() {
    let s = Node {
        count: 1,
        left: Some(Box::new(Node {
            count: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(Node {
            count: 3,
            left: None,
            right: None,
        })),
    };

    let sp = &s;

    if sp.left.as_ref().unwrap().count!= 2 {
        return;
    }

    if s.right.as_ref().unwrap().count!= 3 {
        return;
    }

    if sp.left.as_ref().unwrap() == sp.right.as_ref().unwrap() {
        return;
    }

    // If no return is executed, the program will continue executing and return 0
}