use std::rc::Rc;

struct TNode {
    count: i32,
    left: Option<Rc<TNode>>,
    right: Option<Rc<TNode>>,
}

fn main() {
    let left_child = Rc::new(TNode {
        count: 2,
        left: None,
        right: None,
    });
    let right_child = Rc::new(TNode {
        count: 3,
        left: None,
        right: None,
    });

    let mut s = TNode {
        count: 1,
        left: Some(left_child.clone()),
        right: Some(right_child.clone()),
    };

    let sp = &s;

    if sp.left.as_ref().unwrap().count != 2 {
        std::process::exit(1);
    }

    if s.right.as_ref().unwrap().count != 3 {
        std::process::exit(2);
    }

    if Rc::ptr_eq(sp.left.as_ref().unwrap(), sp.right.as_ref().unwrap()) {
        std::process::exit(3);
    }

    std::process::exit(0);
}