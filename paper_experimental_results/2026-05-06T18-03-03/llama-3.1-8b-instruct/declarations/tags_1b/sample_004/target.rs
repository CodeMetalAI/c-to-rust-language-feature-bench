struct Tnode {
    count: i32,
    left: Option<Box<Tnode>>,
    right: Option<Box<Tnode>>,
}

fn main() {
    let mut s = Tnode {
        count: 1,
        left: None,
        right: None,
    };

    let left_child = Tnode {
        count: 2,
        left: None,
        right: None,
    };

    let right_child = Tnode {
        count: 3,
        left: None,
        right: None,
    };

    s.left = Some(Box::new(left_child));
    s.right = Some(Box::new(right_child));

    let sp = &s;

    if sp.left.as_ref().unwrap().count!= 2 {
        panic!("Expected left child count to be 2");
    }

    if s.right.as_ref().unwrap().count!= 3 {
        panic!("Expected right child count to be 3");
    }

    if sp.left.as_ref().unwrap() == sp.right.as_ref().unwrap() {
        panic!("Left and right children are the same");
    }
}