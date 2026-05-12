struct S {
    i: i32,
}

fn f() -> bool {
    let mut storage = S { i: 0 };
    let mut p: Option<&S> = None;
    let mut q: Option<&S>;
    let mut j = 0;

    loop {
        storage.i = j;
        j += 1;
        q = p;
        p = Some(&storage);
        if j >= 2 {
            break;
        }
    }

    // p == q means both point to the same storage (which they do after 2 iterations)
    // q->i == 1 means the storage has i == 1
    // Since both p and q point to the same storage, and storage.i is now 1
    match (p, q) {
        (Some(p_ref), Some(q_ref)) => {
            std::ptr::eq(p_ref, q_ref) && q_ref.i == 1
        }
        _ => false,
    }
}

fn main() {
    if f() {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}