struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<Box<S>> = None;
    let mut q: Option<Box<S>> = None;
    let mut j = 0;

    loop {
        q = p.take();
        p = Some(Box::new(S { i: j }));
        j += 1;

        if j >= 2 {
            break;
        }
    }

    // Check if p and q point to the same memory (they don't in Rust, but we simulate the check)
    // and if q->i is 1
    matches!((p, q), (Some(p), Some(q)) if p.as_ref() as *const _ == q.as_ref() as *const _ && q.i == 1)
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}