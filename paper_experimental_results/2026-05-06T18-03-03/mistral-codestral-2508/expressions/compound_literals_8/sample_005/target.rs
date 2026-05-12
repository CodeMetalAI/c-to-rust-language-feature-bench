struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<&S> = None;
    let mut q: Option<&S>;
    let mut j = 0;

    loop {
        q = p;
        let s = S { i: j };
        p = Some(&s);
        j += 1;

        if j >= 2 {
            break;
        }
    }

    p.unwrap() == q.unwrap() && q.unwrap().i == 1
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}