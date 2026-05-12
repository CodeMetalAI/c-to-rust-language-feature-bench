fn f(init: i32) -> i32 {
    let mut x = init;
    let p = &mut x;

    let arr: [i32; 2] = [*p, 0];

    if arr[0] != init {
        return 1;
    }
    if arr[1] != 0 {
        return 2;
    }

    0
}

fn main() {
    if f(7) != 0 {
        return 10;
    }
    if f(-3) != 0 {
        return 11;
    }
    0
}