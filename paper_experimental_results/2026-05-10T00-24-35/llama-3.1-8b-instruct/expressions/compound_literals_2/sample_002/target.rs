fn f(init: i32) -> i32 {
    let mut x = init;
    let p = &mut x;

    let array = [x, 0];
    let p = array;

    if p[0] != init {
        return 1;
    }
    if p[1] != 0 {
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