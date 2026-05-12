struct S {
    i: i32,
}

fn f() -> i32 {
    let mut p: Option<&S> = None;
    let mut q: Option<&S>;
    let mut j = 0;

    loop {
        q = p;
        let s_instance = S { i: j };
        p = Some(&s_instance);
        j += 1;
        if j >= 2 {
            break;
        }
    }

    if let (Some(p_ref), Some(q_ref)) = (p, q) {
        if std::ptr::eq(p_ref, q_ref) && q_ref.i == 1 {
            return 1;
        }
    }
    0
}

fn main() {
    std::process::exit(if f() != 0 { 0 } else { 1 });
}