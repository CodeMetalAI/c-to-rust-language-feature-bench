use std::rc::Rc;

#[derive(Clone)]
struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<Rc<S>> = None;
    let mut q: Option<Rc<S>>;
    let mut j = 0;

    loop {
        q = p;
        p = Some(Rc::new(S { i: j }));
        j += 1;
        if j < 2 {
            continue;
        } else {
            break;
        }
    }

    let ptr_eq = match (&p, &q) {
        (Some(p_rc), Some(q_rc)) => Rc::ptr_eq(p_rc, q_rc),
        _ => false,
    };

    ptr_eq && q.as_ref().unwrap().i == 1
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}