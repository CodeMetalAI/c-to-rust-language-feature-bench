use std::rc::Rc;

struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<Rc<S>> = None;
    let mut q: Option<Rc<S>> = None;
    let mut j: i32 = 0;

    loop {
        q = p.clone();
        p = Some(Rc::new(S { i: j }));
        j += 1;
        if j < 2 {
            continue;
        } else {
            break;
        }
    }

    let eq = match (&p, &q) {
        (Some(a), Some(b)) => Rc::ptr_eq(a, b),
        (None, None) => true,
        _ => false,
    };

    if eq {
        if let Some(ref qv) = q {
            qv.i == 1
        } else {
            false
        }
    } else {
        false
    }
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}