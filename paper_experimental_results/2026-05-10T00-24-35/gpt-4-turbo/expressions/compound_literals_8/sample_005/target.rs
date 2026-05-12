fn main() {
    if f() {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

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

        if j >= 2 {
            break;
        }
    }

    if let (Some(p), Some(q)) = (p, q) {
        (std::ptr::eq(&*p, &*q)) && (q.i == 1)
    } else {
        false
    }
}