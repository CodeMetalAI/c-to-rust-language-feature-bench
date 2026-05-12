fn f(init: i32) -> i32 {
    let mut x = init;
    let mut p: &mut [i32] = &mut [x];
    let arr = [p[0], 0];
    p = &mut arr[..];
    if p[0] != init {
        1
    } else if p[1] != 0 {
        2
    } else {
        0
    }
}

fn main() {
    if f(7) != 0 {
        std::process::exit(10);
    }
    if f(-3) != 0 {
        std::process::exit(11);
    }
    std::process::exit(0);
}