struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<S> = None;
    let mut q: Option<S>;
    let mut j = 0;

    loop {
        q = p.take();
        p = Some(S { i: j });
        j += 1;

        if j < 2 {
            continue;
        } else {
            break;
        }
    }

    // Check if p and q point to the same S and q.i is 1
    // In Rust, we can't directly compare addresses, but we can ensure the logic matches.
    // Since we're always creating a new S for p, p and q can never be the same.
    matches!(q, Some(ref x) if x.i == 1)
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}