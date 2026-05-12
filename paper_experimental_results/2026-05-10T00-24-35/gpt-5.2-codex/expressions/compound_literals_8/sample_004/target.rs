use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct S {
    i: i32,
}

fn f() -> i32 {
    let temp = Rc::new(RefCell::new(S { i: 0 }));
    let mut p: Option<Rc<RefCell<S>>> = None;
    let mut q: Option<Rc<RefCell<S>>> = None;
    let mut j: i32 = 0;

    loop {
        q = p.clone();
        {
            let mut t = temp.borrow_mut();
            t.i = j;
        }
        p = Some(temp.clone());
        j += 1;
        if j >= 2 {
            break;
        }
    }

    let result = match (&p, &q) {
        (Some(pp), Some(qq)) => Rc::ptr_eq(pp, qq) && qq.borrow().i == 1,
        _ => false,
    };

    if result { 1 } else { 0 }
}

fn main() {
    let exit_code = if f() != 0 { 0 } else { 1 };
    std::process::exit(exit_code);
}