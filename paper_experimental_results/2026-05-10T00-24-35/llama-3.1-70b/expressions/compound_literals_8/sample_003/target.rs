struct S {
    i: i32,
}

fn f() -> i32 {
    let mut p: Option<&S> = None;
    let mut j = 0;

    loop {
        let q = p.take().map(|s| s as *const S);
        p = Some(&S { i: j });
        j += 1;
        if j >= 2 {
            break;
        }
    }

    let is_equal = p.as_ref().map_or(false, |p| {
        let q = unsafe { &*q.unwrap() };
        p as *const _ == q as *const _ && q.i == 1
    });

    if is_equal {
        0
    } else {
        1
    }
}

fn main() {
    std::process::exit(f());
}