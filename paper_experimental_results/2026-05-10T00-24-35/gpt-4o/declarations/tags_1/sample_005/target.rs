struct TNode {
    count: i32,
    left: Option<Box<TNode>>,
    right: Option<Box<TNode>>,
}

fn main() -> i32 {
    let mut left_child = TNode {
        count: 2,
        left: None,
        right: None,
    };
    
    let mut right_child = TNode {
        count: 3,
        left: None,
        right: None,
    };
    
    let mut s = TNode {
        count: 1,
        left: Some(Box::new(left_child)),
        right: Some(Box::new(right_child)),
    };
    
    let sp = &s;

    if sp.left.as_ref().unwrap().count != 2 {
        return 1;
    }

    if s.right.as_ref().unwrap().count != 3 {
        return 2;
    }

    if std::ptr::eq(sp.left.as_ref().unwrap(), sp.right.as_ref().unwrap()) {
        return 3;
    }

    0
}