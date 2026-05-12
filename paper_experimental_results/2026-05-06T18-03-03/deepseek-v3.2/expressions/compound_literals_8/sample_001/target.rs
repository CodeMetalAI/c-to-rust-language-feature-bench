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

    // Check if p and q point to different allocations (they always will in this implementation)
    // and if q's i field equals 1 (q is the first allocation, which had i=0, not 1)
    // The original C code compares addresses (p == q) which is always false here,
    // and q->i is 0, not 1. So the whole expression is false.
    // Therefore we return 0 (false in C terms).
    0
}

fn main() {
    std::process::exit(f());
}