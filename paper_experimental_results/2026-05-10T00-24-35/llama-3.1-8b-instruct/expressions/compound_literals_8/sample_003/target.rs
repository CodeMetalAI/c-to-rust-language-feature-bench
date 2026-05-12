struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: *const S = std::ptr::null();
    let mut q: *const S;
    let mut j = 0;

    loop {
        q = p;
        p = &S { i: j };
        j += 1;

        if j >= 2 {
            break;
        }
    }

    p == q && (q.i == 1)
}

fn main() {
    println!("{}", f());
}