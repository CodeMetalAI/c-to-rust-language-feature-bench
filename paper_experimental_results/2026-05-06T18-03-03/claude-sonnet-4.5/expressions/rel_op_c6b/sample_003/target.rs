fn main() {
    let np1: Option<&i32> = None;
    let np2: Option<&i32> = None;
    if np1 != np2 {
        std::process::exit(1);
    }

    let s = S { x: 0, y: 0 };
    let ps = &s as *const S as usize;
    let px = &s.x as *const i32 as usize;
    if ps != px {
        std::process::exit(2);
    }

    let a = [1, 2, 3];
    let end1 = a.as_ptr_range().end as usize;
    let end2 = unsafe { a.as_ptr().add(3) } as usize;
    if end1 != end2 {
        std::process::exit(3);
    }

    std::process::exit(0);
}

struct S {
    x: i32,
    y: i32,
}