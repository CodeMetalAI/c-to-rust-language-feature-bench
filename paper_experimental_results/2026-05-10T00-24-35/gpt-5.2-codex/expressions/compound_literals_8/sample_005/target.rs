use std::process::exit;

struct S {
    i: i32,
}

fn f() -> i32 {
    let mut p: Option<Box<S>> = None;
    let mut q: Option<Box<S>> = None;
    let mut j: i32 = 0;

    loop {
        q = p;
        let val = j;
        j += 1;
        p = Some(Box::new(S { i: val }));
        if j < 2 {
            continue;
        }
        break;
    }

    let p_eq_q = match (&p, &q) {
        (Some(pa), Some(qa)) => std::ptr::eq(&**pa, &**qa),
        (None, None) => true,
        _ => false,
    };

    let q_i_eq_1 = match &q {
        Some(qb) => qb.i == 1,
        None => false,
    };

    if p_eq_q && q_i_eq_1 {
        1
    } else {
        0
    }
}

fn main() {
    let result = if f() != 0 { 0 } else { 1 };
    exit(result);
}