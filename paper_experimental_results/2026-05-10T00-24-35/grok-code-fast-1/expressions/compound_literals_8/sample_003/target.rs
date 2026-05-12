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
        q = p.take();
        let new_rc = Rc::new(S { i: j });
        j += 1;
        p = Some(new_rc);
        if j < 2 {
            continue;
        } else {
            break;
        }
    }

    let p_rc = p.unwrap();
    let q_rc = q.unwrap();
    Rc::ptr_eq(&p_rc, &q_rc) && (q_rc.i == 1)
}

fn main() {
    if f() {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}