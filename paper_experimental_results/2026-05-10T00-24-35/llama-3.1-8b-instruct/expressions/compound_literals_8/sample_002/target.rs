struct S {
    i: i32,
}

fn f() -> bool {
    let mut p = None;
    let mut j = 0;

    loop {
        let q = p.take().or_else(|| Some(S { i: j }));
        p = Some(S { i: j });
        j += 1;

        if j < 2 {
            continue;
        }

        return p == q && p.map(|s| s.i).unwrap() == 1;
    }
}

fn main() {
    println!("{}", f());
}