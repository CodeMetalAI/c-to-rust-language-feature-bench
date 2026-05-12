struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<&S> = None;
    let mut q: Option<&S>;
    let mut j = 0;

    'again: loop {
        q = p;
        p = Some(&S { i: j });
        j += 1;

        if j < 2 {
            continue 'again;
        }

        break;
    }

    p.unwrap() as *const S == q.unwrap() as *const S && q.unwrap().i == 1
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}