struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<&mut S> = None;
    let mut q: Option<&mut S> = None;
    let mut j = 0;

    while j < 2 {
        q = p;
        p = Some(&mut S { i: j });
        j += 1;
    }

    p == q && p.unwrap().i == 1
}

fn main() -> i32 {
    f().then(|| 0).unwrap_or(1)
}