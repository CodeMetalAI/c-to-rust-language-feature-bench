struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<&S> = None;
    let mut q: Option<&S>;
    let mut j = 0;

    loop {
        q = p;
        p = Some(&S { i: j });
        j += 1;
        if j >= 2 {
            break;
        }
    }

    p == q && q.unwrap().i == 1
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}