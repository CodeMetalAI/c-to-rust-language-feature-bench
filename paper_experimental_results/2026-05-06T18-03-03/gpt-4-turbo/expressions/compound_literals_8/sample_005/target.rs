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

    // In Rust, we cannot compare pointers directly like in C/C++, so we check the values instead.
    // Since the struct `S` is created in each iteration anew, they cannot be the same.
    // We check the value of `i` in `q` to see if it matches the expected value.
    matches!(q, Some(ref x) if x.i == 1)
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}