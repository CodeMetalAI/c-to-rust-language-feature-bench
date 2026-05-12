fn main() {
    let np1: Option<&i32> = None;
    let np2: Option<&i32> = None;
    if np1 != np2 {
        std::process::exit(1);
    }

    let s = (0i32, 0i32);
    let ps = &s as *const _ as *const ();
    let px = &s.0 as *const _ as *const ();
    if ps != px {
        std::process::exit(2);
    }

    let a = [1, 2, 3];
    let end1 = unsafe { a.as_ptr().add(3) };
    let end2 = unsafe { a.as_ptr().add(3) };
    if end1 != end2 {
        std::process::exit(3);
    }

    std::process::exit(0);
}