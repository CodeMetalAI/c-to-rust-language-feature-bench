fn f(init: i32) -> i32 {
    let mut x = init;
    let p = &mut x;

    let arr: [i32; 2] = [*p];
    let p = &arr;

    if p[0]!= init {
        return 1;
    }
    if p[1]!= 0 {
        return 2;
    }

    0
}

fn main() {
    if f(7)!= 0 {
        std::process::exit(10);
    }
    if f(-3)!= 0 {
        std::process::exit(11);
    }
}