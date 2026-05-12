fn f1() -> i32 {
    1
}

fn f2(t: i32) -> i32 {
    t + 1
}

fn main() {
    let mut t: i32 = 10;
    let val = {
        t = f1();
        f2(t)
    };
    let exit_code = if val == 2 { 0 } else { 1 };
    std::process::exit(exit_code);
}