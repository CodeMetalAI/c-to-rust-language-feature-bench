struct S {
    i: i32,
}

fn f() -> i32 {
    let mut j = 0;
    let mut p: Option<Box<S>> = None;
    let mut q: Option<Box<S>> = None;

    loop {
        q = p;
        p = Some(Box::new(S { i: j }));
        j += 1;
        if j >= 2 {
            break;
        }
    }

    // Check if p and q are both Some and compare their values
    match (p, q) {
        (Some(p_box), Some(q_box)) => {
            // Compare addresses by comparing the Box pointers (via std::ptr::eq)
            let ptr_eq = std::ptr::eq(p_box.as_ref(), q_box.as_ref());
            // Compare the i field
            let val_eq = q_box.i == 1;
            (ptr_eq && val_eq) as i32
        }
        _ => 0,
    }
}

fn main() {
    std::process::exit(f());
}