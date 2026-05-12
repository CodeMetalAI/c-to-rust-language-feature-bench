struct S {
    x: i32,
    y: i32,
}

fn main() {
    let np1: *const i32 = std::ptr::null();
    let np2: *const i32 = std::ptr::null();
    if np1 as usize!= np2 as usize {
        return;
    }

    let s = S { x: 0, y: 0 };
    let ps: *const u8 = &s as *const S as *const u8;
    let px: *const u8 = &s.x as *const i32 as *const u8;
    if ps as usize!= px as usize {
        std::process::exit(2);
    }

    let a = [1, 2, 3];
    let end1 = &a[3] as *const i32;
    let end2 = a.as_ptr().add(3);
    if end1 as usize!= end2 as usize {
        std::process::exit(3);
    }

    std::process::exit(0);
}