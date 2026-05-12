use std::rc::Rc;

struct S {
    i: i32,
}

fn f() -> i32 {
    let mut p: Option<Rc<S>> = None;
    let mut q: Option<Rc<S>> = None;
    let mut j: i32 = 0;

    loop {
        q = p.clone();
        let new = Rc::new(S { i: j });
        j += 1;
        p = Some(new);

        if j < 2 {
            continue;
        } else {
            break;
        }
    }

    let result = if let (Some(pv), Some(qv)) = (p.as_ref(), q.as_ref()) {
        Rc::ptr_eq(pv, qv) && qv.i == 1
    } else {
        false
    };

    if result { 1 } else { 0 }
}

fn main() {
    let code = if f() != 0 { 0 } else { 1 };
    std::process::exit(code);
}