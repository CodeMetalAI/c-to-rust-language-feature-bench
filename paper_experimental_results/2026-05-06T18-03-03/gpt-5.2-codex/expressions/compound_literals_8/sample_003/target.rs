#[derive(Clone, Copy)]
struct S {
    i: i32,
}

fn f() -> bool {
    let mut vec: Vec<S> = Vec::new();
    let mut p: Option<usize> = None;
    let mut q: Option<usize>;
    let mut j: i32 = 0;

    loop {
        q = p;
        vec.push(S { i: j });
        p = Some(vec.len() - 1);
        j += 1;
        if j < 2 {
            continue;
        } else {
            break;
        }
    }

    if p == q {
        if let Some(idx) = q {
            vec[idx].i == 1
        } else {
            false
        }
    } else {
        false
    }
}

fn main() {
    let code = if f() { 0 } else { 1 };
    std::process::exit(code);
}