struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<&S> = None;
    let mut q: Option<&S> = None;
    let mut j: i32 = 0;

    loop {
        q = p;
        p = Some(&S { i: j });
        j += 1;
        if j < 2 {
            continue;
        } else {
            break;
        }
    }

    p == q && q.map_or(false, |q| q.i == 1)
}

fn main() {
    if f() {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}