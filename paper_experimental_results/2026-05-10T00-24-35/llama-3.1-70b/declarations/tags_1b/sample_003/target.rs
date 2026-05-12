#[derive(Debug)]
struct TNode {
    count: i32,
    left: Option<Box<TNode>>,
    right: Option<Box<TNode>>,
}

impl TNode {
    fn new() -> Self {
        TNode {
            count: 0,
            left: None,
            right: None,
        }
    }
}

fn main() {
    let mut s = TNode::new();
    let mut left_child = TNode::new();
    let mut right_child = TNode::new();

    s.count = 1;
    left_child.count = 2;
    right_child.count = 3;

    s.left = Some(Box::new(left_child));
    s.right = Some(Box::new(right_child));

    if s.left.as_ref().unwrap().count != 2 {
        return;
    }

    if s.right.as_ref().unwrap().count != 3 {
        return;
    }

    if s.left.as_ref() == s.right.as_ref() {
        return;
    }

    println!("All tests passed");
}