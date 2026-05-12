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

    // Check if p and q point to different allocations (they always will in this case)
    // and check if q's i field equals 1
    if let (Some(p_box), Some(q_box)) = (&p, &q) {
        // Compare addresses by comparing the boxes' contents
        // Since each Box is a unique allocation, they are never equal
        // In the C version, p == q compares pointers, which will be false
        // because each compound literal has a different address.
        // The C code returns 1 only if p == q AND q->i == 1.
        // Since p != q always, the whole expression is 0.
        // But wait: let's trace the C code more carefully.
        // In C: j starts at 0, p = 0, q uninitialized.
        // First iteration: q = p (q = 0), p = &((struct s){0}), j becomes 1.
        // Since j < 2, goto again.
        // Second iteration: q = p (q = &first literal), p = &((struct s){1}), j becomes 2.
        // Now j >= 2, exit loop.
        // p points to second literal, q points to first literal.
        // p != q, so (p == q) is false, so whole expression is 0.
        // Therefore f() returns 0.
        // So our Rust version should return 0.
        // But the C code returns (p == q) && (q->i == 1).
        // Since p != q, it's false regardless of q->i.
        // However, if we wanted to check q->i == 1: first literal had i=0, second had i=1.
        // q points to first literal (i=0), not second.
        // So q->i == 1 is false too.
        // Overall false -> 0.
        
        // In Rust, we can't directly compare addresses of Boxes for equality
        // in the same way as C pointers. But we can compare if they point to
        // the same allocation using std::ptr::eq.
        use std::ptr;
        let p_eq_q = ptr::eq(p_box.as_ref(), q_box.as_ref());
        (p_eq_q && q_box.i == 1) as i32
    } else {
        0
    }
}

fn main() {
    std::process::exit(f());
}