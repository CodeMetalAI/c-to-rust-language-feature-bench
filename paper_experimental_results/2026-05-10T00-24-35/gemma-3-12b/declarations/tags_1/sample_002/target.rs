#[derive(Debug)]
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

    if let Some(ref left) = sp.left {
        if left.count != 2 {
            return 1;
        }
    } else {
        return 1;
    }

    if let Some(ref right) = sp.right {
        if right.count != 3 {
            return 2;
        }
    } else {
        return 2;
    }

    if sp.left.as_ref().map(|x| x.as_ref()).eq(&sp.right.as_ref().map(|x| x.as_ref())) {
        return 3;
    }

    0
}