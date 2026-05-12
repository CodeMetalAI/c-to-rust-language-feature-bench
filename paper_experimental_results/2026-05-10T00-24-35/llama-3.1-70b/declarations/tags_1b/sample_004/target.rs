struct Node {
    count: i32,
    left: *mut Node,
    right: *mut Node,
}

fn main() {
    let mut s = Node {
        count: 1,
        left: std::ptr::null_mut(),
        right: std::ptr::null_mut(),
    };

    let mut left_child = Node {
        count: 2,
        left: std::ptr::null_mut(),
        right: std::ptr::null_mut(),
    };

    let mut right_child = Node {
        count: 3,
        left: std::ptr::null_mut(),
        right: std::ptr::null_mut(),
    };

    s.left = &mut left_child as *mut Node;
    s.right = &mut right_child as *mut Node;

    let sp = &mut s as *mut Node;

    if unsafe { (*sp).left.as_ref().unwrap().count } != 2 {
        return;
    }

    if s.right.as_ref().unwrap().count != 3 {
        return;
    }

    if unsafe { (*sp).left } == unsafe { (*sp).right } {
        return;
    }

    println!("Success");
}