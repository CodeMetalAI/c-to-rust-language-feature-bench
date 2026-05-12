fn main() -> i32 {
    let x = 7;
    let p: &i32 = &x;

    if &x as *const _ != p as *const _ {
        return 1;
    }

    if &(*p) as *const _ != p as *const _ {
        return 2;
    }

    let a = [10, 20, 30];

    if &a[0] as *const _ != &a[0] as *const _ {
        return 3;
    }
    if &a[1] as *const _ != &a[1] as *const _ {
        return 4;
    }
    if &a[2] as *const _ != &a[2] as *const _ {
        return 5;
    }

    if a[1] != 20 {
        return 6;
    }

    0
}