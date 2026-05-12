struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<&S> = None;
    let mut j = 0;

    loop {
        let q = p;
        p = Some(&S { i: j });
        j += 1;

        if j >= 2 {
            break;
        }
    }

    p == q && q.map_or(false, |x| x.i == 1)
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}