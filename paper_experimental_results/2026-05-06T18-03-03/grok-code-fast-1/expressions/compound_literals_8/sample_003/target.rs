use std::rc::Rc;

#[derive(Debug)]
struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<Rc<S>> = None;
    let mut q: Option<Rc<S>>;
    let mut j = 0;
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
    p == q && q.as_ref().unwrap().i == 1
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}