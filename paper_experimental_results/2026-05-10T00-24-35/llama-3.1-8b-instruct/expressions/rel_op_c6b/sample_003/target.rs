struct S {
    x: i32,
    y: i32,
}

fn main() {
    let np1 = std::ptr::null::<i32>();
    let np2 = std::ptr::null::<i32>();
    if np1 != np2 {
        return 1;
    }

    let mut s = S { x: 0, y: 0 };
    let ps = &s as *const _;
    let px: *const i32 = &s.x;
    if ps != px {
        return 2;
    }

    let a: [i32; 3] = [1, 2, 3];
    let end1 = &a[3];
    let end2 = unsafe { a.as_ptr().offset(3) };
    if end1 != end2 {
        return 3;
    }

    println!("{}", 0);
}