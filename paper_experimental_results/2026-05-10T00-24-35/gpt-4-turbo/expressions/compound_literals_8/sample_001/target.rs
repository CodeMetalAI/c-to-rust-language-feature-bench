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

    let same_address = std::ptr::eq(p.as_ref().unwrap().as_ref(), q.as_ref().unwrap().as_ref());
    let correct_value = q.unwrap().i == 1;
    same_address && correct_value
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}