use std::cell::RefCell;
use std::rc::Rc;
use std::process::exit;

struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<Rc<RefCell<S>>> = None;
    let mut q: Option<Rc<RefCell<S>>> = None;
    let mut j: i32 = 0;

    let temp = Rc::new(RefCell::new(S { i: 0 }));

    loop {
        q = p.clone();
        {
            let mut t = temp.borrow_mut();
            t.i = j;
        }
        j += 1;
        p = Some(temp.clone());

        if j < 2 {
            continue;
        }
        break;
    }

    if let (Some(p_rc), Some(q_rc)) = (&p, &q) {
        Rc::ptr_eq(p_rc, q_rc) && q_rc.borrow().i == 1
    } else {
        false
    }
}

fn main() {
    if f() {
        exit(0);
    } else {
        exit(1);
    }
}