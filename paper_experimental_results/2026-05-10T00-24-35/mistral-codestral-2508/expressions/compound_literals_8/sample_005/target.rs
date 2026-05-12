struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<&S> = None;
    let mut q: Option<&S> = None;
    let mut j = 0;

    loop {
        q = p;
        p = Some(&S { i: j });
        j += 1;

        if j >= 2 {
            break;
        }
    }

    p.is_some() && q.is_some() && p.unwrap().i == q.unwrap().i && q.unwrap().i == 1
}

fn main() {
    if f() {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}