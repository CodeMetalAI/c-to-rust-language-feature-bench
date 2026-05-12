use std::rc::Rc;

#[derive(Debug, PartialEq)]
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
    match (&p, &q) {
        (Some(pp), Some(qq)) => Rc::ptr_eq(pp, qq) && qq.i == 1,
        _ => false,
    }
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}