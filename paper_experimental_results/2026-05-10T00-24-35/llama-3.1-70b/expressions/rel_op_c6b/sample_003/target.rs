fn main() {
    let np1: *const i32 = std::ptr::null();
    let np2: *const i32 = std::ptr::null();
    if np1!= np2 {
        return;
    }

    struct S {
        x: i32,
        y: i32,
    };

    let s = S { x: 0, y: 0 };
    let ps: *const S = &s;
    let px: *const i32 = &s.x;
    if ps as *const u8!= px as *const u8 {
        return;
    }

    let a = [1, 2, 3];
    let end1: *const i32 = unsafe { a.as_ptr().offset(3) };
    let end2: *const i32 = unsafe { a.as_ptr().add(3) };
    if end1!= end2 {
        return;
    }
}