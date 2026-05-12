#[derive(Debug)]
struct TNode {
    count: i32,
    left: Option<Box<TNode>>,
    right: Option<Box<TNode>>,
}

fn main() {
    let mut s = TNode {
        count: 1,
        left: None,
        right: None,
    };

    let left_child = TNode {
        count: 2,
        left: None,
        right: None,
    };

    let right_child = TNode {
        count: 3,
        left: None,
        right: None,
    };

    s.left = Some(Box::new(left_child));
    s.right = Some(Box::new(right_child));

    let sp = &s;

    if sp.left.as_ref().unwrap().count!= 2 {
        std::process::exit(1);
    }

    if s.right.as_ref().unwrap().count!= 3 {
        std::process::exit(2);
    }

    if sp.left.as_ref().unwrap() as *const _ == sp.right.as_ref().unwrap() as *const _ {
        std::process::exit(3);
    }

    std::process::exit(0);
}