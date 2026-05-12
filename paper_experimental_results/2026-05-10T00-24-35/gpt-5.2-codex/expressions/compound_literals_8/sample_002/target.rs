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
        let new_obj = Rc::new(S { i: j });
        j += 1;
        p = Some(new_obj);
        if j >= 2 {
            break;
        }
    }

    match (&p, &q) {
        (Some(pv), Some(qv)) => Rc::ptr_eq(pv, qv) && qv.i == 1,
        _ => false,
    }
}

fn main() {
    let code = if f() { 0 } else { 1 };
    std::process::exit(code);
}