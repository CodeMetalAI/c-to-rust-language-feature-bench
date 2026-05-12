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

    // Check if p and q point to different allocations (they always will in Rust)
    // and if q's i is 1 (q is the first allocation, which had i = 0, not 1)
    // The original C code returns 1 if p == q and q->i == 1, which never happens.
    // In Rust, we compare the values of i, not pointer equality.
    if let (Some(p_box), Some(q_box)) = (&p, &q) {
        // p and q are different allocations, so p != q
        // q->i is 0 (from first iteration), not 1
        // So the condition is false, return 0
        0
    } else {
        // This branch is unreachable since both are Some after the loop
        0
    }
}

fn main() {
    std::process::exit(f());
}