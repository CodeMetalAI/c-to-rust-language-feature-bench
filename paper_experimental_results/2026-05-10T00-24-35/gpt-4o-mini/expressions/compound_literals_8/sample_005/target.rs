struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<Box<S>> = None;
    let mut q: Option<Box<S>>;
    let mut j = 0;

    loop {
        q = p.take();
        p = Some(Box::new(S { i: j }));
        j += 1;

        if j >= 2 {
            break;
        }
    }

    p.as_ref() == q.as_ref() && q.as_ref().map_or(false, |v| v.i == 1)
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}