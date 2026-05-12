struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<&S> = None;
    let mut j = 0;

    loop {
        let q = p.take();
        p = Some(&S { i: j });
        j += 1;

        if j >= 2 {
            break;
        }
    }

    p.is_none() && q.map_or(false, |s| s.i == 1)
}

fn main() {
    println!("{}", f().then(|| 0).unwrap_or(1));
}