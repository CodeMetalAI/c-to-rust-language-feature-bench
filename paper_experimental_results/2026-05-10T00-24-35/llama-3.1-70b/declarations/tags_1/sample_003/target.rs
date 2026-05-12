fn main() {
    struct TNode {
        count: i32,
        left: Option<&'static TNode>,
        right: Option<&'static TNode>,
    }

    static mut LEFT_CHILD: TNode = TNode {
        count: 2,
        left: None,
        right: None,
    };

    static mut RIGHT_CHILD: TNode = TNode {
        count: 3,
        left: None,
        right: None,
    };

    let mut tnode: TNode = TNode {
        count: 1,
        left: Some(&LEFT_CHILD),
        right: Some(&RIGHT_CHILD),
    };

    let tnode_ptr = &tnode;

    if tnode_ptr.left.unwrap().count != 2 {
        std::process::exit(1);
    }

    if tnode.right.unwrap().count != 3 {
        std::process::exit(2);
    }

    if tnode_ptr.left == tnode_ptr.right {
        std::process::exit(3);
    }

    std::process::exit(0);
}