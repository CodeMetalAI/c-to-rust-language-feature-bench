// compound_literal_8

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
        if j < 2 {
            continue;
        }

        break;
    }

    p == q && p.map(|s| s.i == 1).unwrap_or(false)
}

fn main() -> i32 {
    f() as i32
}