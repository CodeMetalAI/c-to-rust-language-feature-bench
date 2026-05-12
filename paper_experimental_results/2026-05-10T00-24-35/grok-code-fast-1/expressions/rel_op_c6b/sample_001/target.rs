struct S {
    x: i32,
    y: i32,
}

fn main() {
    let np1: usize = 0;
    let np2: usize = 0;
    if np1 != np2 {
        std::process::exit(1);
    }

    let s = S { x: 0, y: 0 };
    let ps: usize = (&s as *const S).cast::<()> as usize;
    let px: usize = (&s.x as *const i32).cast::<()> as usize;
    if ps != px {
        std::process::exit(2);
    }

    let a: [i32; 3] = [1, 2, 3];
    let base: usize = a.as_ptr() as usize;
    let end1: usize = base + 3 * std::mem::size_of::<i32>();
    let end2: usize = base + 3 * std::mem::size_of::<i32>();
    if end1 != end2 {
        std::process::exit(3);
    }

    std::process::exit(0);
}