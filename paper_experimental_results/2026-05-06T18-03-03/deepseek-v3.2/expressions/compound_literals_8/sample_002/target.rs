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

    // Check if p and q point to different allocations (they always will in this code)
    // and check if q's i is 1 (since j was incremented after creating the first S)
    if let (Some(p_ref), Some(q_ref)) = (&p, &q) {
        // In the C code, p != q because they point to different compound literals
        // and q->i == 1 because the second iteration creates S with i=1
        (p_ref.as_ref() as *const S != q_ref.as_ref() as *const S && q_ref.i == 1) as i32
    } else {
        0
    }
}

fn main() {
    std::process::exit(f());
}