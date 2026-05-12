struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<&S> = None;
    let mut q: Option<&S>;
    let mut j = 0;

    loop {
        let new_s = S { i: j };
        j += 1;
        q = p;
        p = Some(&new_s);
        if j >= 2 {
            break;
        }
    }

    // Since `p` and `q` are references to temporaries that do not live beyond the loop,
    // they cannot be compared directly for equality of their addresses.
    // We compare the values they point to instead.
    matches!(p, Some(p_val) if matches!(q, Some(q_val) if p_val as *const _ == q_val as *const _ && q_val.i == 1))
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}