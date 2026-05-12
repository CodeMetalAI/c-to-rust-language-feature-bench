fn main() -> i32 {
    let x = 7;
    let p = &x;

    if &x as *const i32 != p as *const i32 {
        return 1;
    }

    if &(*p) as *const i32 != p as *const i32 {
        return 2;
    }

    let a = [10, 20, 30];

    if &a[0] as *const i32 != a.as_ptr().wrapping_add(0) {
        return 3;
    }
    if &a[1] as *const i32 != a.as_ptr().wrapping_add(1) {
        return 4;
    }
    if &a[2] as *const i32 != a.as_ptr().wrapping_add(2) {
        return 5;
    }

    if unsafe { *a.as_ptr().wrapping_add(1) } != 20 {
        return 6;
    }

    0
}