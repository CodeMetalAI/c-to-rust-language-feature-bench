use std::boxed::Box;

struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<Box<S>> = None;
    let mut q: Option<Box<S>> = None;
    let mut j = 0;
    loop {
        q = p.take();
        p = Some(Box::new(S { i: j }));
        j += 1;
        if j < 2 {
            continue;
        } else {
            break;
        }
    }
    if let (Some(ref pp), Some(ref qq)) = (&p, &q) {
        (pp.as_ptr() == qq.as_ptr()) && (qq.i == 1)
    } else {
        false
    }
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}