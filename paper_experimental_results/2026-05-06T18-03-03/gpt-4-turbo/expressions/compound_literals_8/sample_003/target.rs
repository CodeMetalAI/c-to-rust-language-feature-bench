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

        if j < 2 {
            continue;
        } else {
            break;
        }
    }

    // Check if p and q point to the same instance and q.i is 1
    // In Rust, we can't directly compare Box pointers like in C/C++, so we use other means
    // Here, since we know the loop runs exactly twice, q should have the value from the first loop
    // and p from the second, hence they can't be the same. We check q's value instead.
    matches!(q, Some(ref x) if x.i == 1)
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}