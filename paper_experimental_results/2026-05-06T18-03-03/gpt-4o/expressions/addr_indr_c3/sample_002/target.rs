fn main() -> i32 {
    let x = 7;
    let p = &x;

    if &x != p {
        return 1;
    }

    if &(*p) != p {
        return 2;
    }

    let a = [10, 20, 30];

    if &a[0] != a.as_ptr().wrapping_add(0) {
        return 3;
    }
    if &a[1] != a.as_ptr().wrapping_add(1) {
        return 4;
    }
    if &a[2] != a.as_ptr().wrapping_add(2) {
        return 5;
    }

    if unsafe { *a.as_ptr().add(1) } != 20 {
        return 6;
    }

    0
}