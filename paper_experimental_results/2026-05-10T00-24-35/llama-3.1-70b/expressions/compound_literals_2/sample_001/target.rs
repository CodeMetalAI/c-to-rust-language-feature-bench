fn f(init: i32) -> i32 {
    let mut x = init;
    let p = &[x];

    if p[0] != init {
        return 1;
    }
    if p.len() < 2 || p[1] != 0 {
        return 2;
    }

    0
}

fn main() {
    if f(7) != 0 {
        std::process::exit(10);
    }
    if f(-3) != 0 {
        std::process::exit(11);
    }
}