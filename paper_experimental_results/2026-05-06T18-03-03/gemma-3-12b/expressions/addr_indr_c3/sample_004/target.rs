fn main() {
    let mut x = 7;
    let p = &x;

    if &x as *const i32 != p as *const i32 {
        return 1;
    }

    if &(*p) as *const i32 != p as *const i32 {
        return 2;
    }

    let a = [10, 20, 30];

    if &a[0] as *const i32 != &a as *const [i32] as *const i32 + 0 {
        return 3;
    }
    if &a[1] as *const i32 != &a as *const [i32] as *const i32 + 1 {
        return 4;
    }
    if &a[2] as *const i32 != &a as *const [i32] as *const i32 + 2 {
        return 5;
    }

    if (a.as_ptr() as *const i32 + 1) != &a[1] as *const i32 {
        return 6;
    }

    if *(a.as_ptr() + 1) != 20 {
        return 6;
    }

    0
}