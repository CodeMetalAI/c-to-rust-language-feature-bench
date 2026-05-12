struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<&S> = None;
    let mut q: Option<&S> = None;
    let mut j = 0;

    loop {
        q = p;
        p = Some(&S { i: j });
        j += 1;
        if j < 2 {
            continue;
        }
        break;
    }

    p == q && p.as_ref().unwrap().i == 1
}

fn main() {
    println!("{}", if f() { 0 } else { 1 });
}