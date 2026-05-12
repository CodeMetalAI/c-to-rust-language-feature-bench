fn f(init: i32) -> i32 {
    let mut p = vec![init];
    p = vec![p[0], 0];
    if p[0] != init {
        return 1;
    }
    if p[1] != 0 {
        return 2;
    }
    0
}

fn main() -> i32 {
    if f(7) != 0 {
        return 10;
    }
    if f(-3) != 0 {
        return 11;
    }
    0
}