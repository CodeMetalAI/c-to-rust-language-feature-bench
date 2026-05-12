struct S {
    i: i32,
}

fn f() -> bool {
    let mut structs: Vec<S> = Vec::new();
    let mut p: Option<usize> = None;
    let mut q: Option<usize>;
    let mut j = 0;

    loop {
        q = p;
        let idx = structs.len();
        structs.push(S { i: j });
        p = Some(idx);
        j += 1;
        if j >= 2 {
            break;
        }
    }

    p == q && q.map_or(false, |idx| structs[idx].i == 1)
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}