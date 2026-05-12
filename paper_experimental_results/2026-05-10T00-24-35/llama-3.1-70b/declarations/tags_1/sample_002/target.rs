fn main() {
    struct Tnode {
        count: i32,
        left: Option<&'static Tnode>,
        right: Option<&'static Tnode>,
    }

    static mut LEFT_CHILD: Tnode = Tnode {
        count: 2,
        left: None,
        right: None,
    };

    static mut RIGHT_CHILD: Tnode = Tnode {
        count: 3,
        left: None,
        right: None,
    };

    let mut s = Tnode {
        count: 1,
        left: Some(&LEFT_CHILD),
        right: Some(&RIGHT_CHILD),
    };

    let sp = &s;

    if sp.left.unwrap().count != 2 {
        std::process::exit(1);
    }

    if s.right.unwrap().count != 3 {
        std::process::exit(2);
    }

    if sp.left.unwrap() as *const Tnode == sp.right.unwrap() as *const Tnode {
        std::process::exit(3);
    }

    std::process::exit(0);
}