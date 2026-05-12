struct TNode {
    count: i32,
    left: Option<&'static TNode>,
    right: Option<&'static TNode>,
}

fn main() {
    static mut left_child: TNode = TNode {
        count: 2,
        left: None,
        right: None,
    };
    static mut right_child: TNode = TNode {
        count: 3,
        left: None,
        right: None,
    };
    static mut s: TNode = TNode {
        count: 1,
        left: Some(&left_child),
        right: Some(&right_child),
    };

    let sp = &s;

    if sp.left.unwrap().count != 2 {
        std::process::exit(1);
    }

    if s.right.unwrap().count != 3 {
        std::process::exit(2);
    }

    if sp.left.unwrap() as *const _ == sp.right.unwrap() as *const _ {
        std::process::exit(3);
    }

    std::process::exit(0);
}