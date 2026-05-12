struct S {
    i: i32,
}

fn f() -> bool {
    let mut j = 0;
    let mut p: Option<&S> = None;
    let mut q: Option<&S> = None;

    while j < 2 {
        q = p;
        p = Some(&S { i: j });
        j += 1;
    }

    p == q && q.map(|s| s.i == 1).unwrap_or(false)
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}