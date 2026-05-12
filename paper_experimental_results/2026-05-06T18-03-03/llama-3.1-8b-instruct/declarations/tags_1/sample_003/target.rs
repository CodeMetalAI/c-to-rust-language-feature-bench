struct TNode {
    count: i32,
    left: Option<Box<TNode>>,
    right: Option<Box<TNode>>,
}

fn main() {
    let s = TNode {
        count: 1,
        left: Some(Box::new(TNode {
            count: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TNode {
            count: 3,
            left: None,
            right: None,
        })),
    };

    let sp = &s;

    if sp.left.as_ref().unwrap().count != 2 {
        return;
    }

    if s.right.as_ref().unwrap().count != 3 {
        return;
    }

    if sp.left.as_ref().unwrap() == sp.right.as_ref().unwrap() {
        return;
    }

    println!("0");
}