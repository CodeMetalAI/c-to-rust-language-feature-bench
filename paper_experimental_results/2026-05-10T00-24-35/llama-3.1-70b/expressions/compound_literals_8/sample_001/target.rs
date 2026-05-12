struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: *mut S = std::ptr::null_mut();
    let mut q: *mut S = std::ptr::null_mut();
    let mut j: i32 = 0;

    loop {
        q = p;
        p = Box::into_raw(Box::new(S { i: j }));
        j += 1;
        if j < 2 {
            continue;
        } else {
            break;
        }
    }

    let q_value = unsafe { (*q).i };
    let p_value = unsafe { (*p).i };
    (p == q) && (q_value == 1)
}

fn main() {
    if f() {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}