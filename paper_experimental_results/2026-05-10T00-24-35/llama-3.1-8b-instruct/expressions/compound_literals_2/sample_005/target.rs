fn f(init: i32) -> i32 {
    let mut x = init;
    let mut p = &mut x;

    let array: [i32; 2] = [*p, 0];

    if array[0]!= init {
        return 1;
    }
    if array[1]!= 0 {
        return 2;
    }

    return 0;
}

fn main() {
    if f(7)!= 0 {
        return 10;
    }
    if f(-3)!= 0 {
        return 11;
    }
    return 0;
}