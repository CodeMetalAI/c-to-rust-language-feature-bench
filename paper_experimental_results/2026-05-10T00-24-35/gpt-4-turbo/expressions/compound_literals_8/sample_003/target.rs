struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<Box<S>> = None;
    let mut q: Option<Box<S>>;
    let mut j = 0;

    loop {
        q = p;
        p = Some(Box::new(S { i: j }));
        j += 1;
        if j >= 2 {
            break;
        }
    }

    // Check pointer equality and value equality
    std::ptr::eq(p.as_ref().unwrap(), q.as_ref().unwrap()) && (q.as_ref().unwrap().i == 1)
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}