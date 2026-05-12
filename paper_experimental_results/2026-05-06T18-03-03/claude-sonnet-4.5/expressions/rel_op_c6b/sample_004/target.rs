fn main() {
    let np1: Option<&i32> = None;
    let np2: Option<&i32> = None;
    if np1 != np2 {
        std::process::exit(1);
    }

    let s = (0i32, 0i32);
    let ps = &s as *const _ as usize;
    let px = &s.0 as *const _ as usize;
    if ps != px {
        std::process::exit(2);
    }

    let a = [1i32, 2, 3];
    let end1 = unsafe { a.as_ptr().add(3) as usize };
    let end2 = unsafe { a.as_ptr().add(3) as usize };
    if end1 != end2 {
        std::process::exit(3);
    }

    std::process::exit(0);
}