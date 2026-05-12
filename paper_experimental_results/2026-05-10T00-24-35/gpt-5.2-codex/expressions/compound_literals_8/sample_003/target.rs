use std::process;

struct S {
    i: i32,
}

fn f() -> i32 {
    let mut p: Option<usize> = None;
    let mut q: Option<usize> = None;
    let mut j: i32 = 0;
    let mut store: Vec<S> = Vec::new();

    loop {
        q = p;
        store.push(S { i: j });
        p = Some(store.len() - 1);
        j += 1;
        if j < 2 {
            continue;
        } else {
            break;
        }
    }

    let cond = match (p, q) {
        (Some(pi), Some(qi)) => (pi == qi) && (store[qi].i == 1),
        _ => false,
    };

    if cond { 1 } else { 0 }
}

fn main() {
    let exit_code = if f() != 0 { 0 } else { 1 };
    process::exit(exit_code);
}