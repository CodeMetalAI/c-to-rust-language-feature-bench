#[derive(Debug, PartialEq)]
struct Tnode {
    count: i32,
    left: Option<&'static Tnode>,
    right: Option<&'static Tnode>,
}

fn main() {
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
    let mut s = Tnode {
        count: 1,
        left: None,
        right: None,
    };
    s.left = Some(&left_child);
    s.right = Some(&right_child);

    let sp = &s;

    if sp.left.unwrap().count != 2 {
        std::process::exit(1);
    }

    if s.right.unwrap().count != 3 {
        std::process::exit(2);
    }

    if sp.left == sp.right {
        std::process::exit(3);
    }

    std::process::exit(0);
}