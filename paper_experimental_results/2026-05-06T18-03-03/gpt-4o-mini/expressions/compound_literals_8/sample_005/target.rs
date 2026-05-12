struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<Box<S>> = None;
    let mut j = 0;

    loop {
        let q = p.take();
        p = Some(Box::new(S { i: j }));
        j += 1;

        if j >= 2 {
            break;
        }
    }

    p.as_ref().map_or(false, |p_ref| {
        let q_ref = p.as_ref().unwrap();
        p_ref.i == 1 && q_ref.i == 1
    })
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}