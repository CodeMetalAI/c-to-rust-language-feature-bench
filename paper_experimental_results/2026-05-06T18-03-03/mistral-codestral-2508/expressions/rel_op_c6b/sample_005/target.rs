fn main() {
    let np1: *const i32 = std::ptr::null();
    let np2: *const i32 = std::ptr::null();
    if !np1.eq(&np2) {
        std::process::exit(1);
    }

    #[repr(C)]
    struct S {
        x: i32,
        y: i32,
    }

    let s = S { x: 0, y: 0 };
    let ps = &s as *const S as *const ();
    let px = &s.x as *const i32 as *const ();
    if !ps.eq(&px) {
        std::process::exit(2);
    }

    let a = [1, 2, 3];
    let end1 = &a[3] as *const i32;
    let end2 = a.as_ptr().wrapping_add(3);
    if !end1.eq(&end2) {
        std::process::exit(3);
    }
}