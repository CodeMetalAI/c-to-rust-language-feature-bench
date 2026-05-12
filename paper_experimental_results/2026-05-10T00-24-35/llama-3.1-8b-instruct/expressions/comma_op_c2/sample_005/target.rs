fn f1() -> i32 {
    1
}

fn f2(t: i32) -> i32 {
    t + 1
}

fn main() {
    let mut t: i32 = 10;
    let val = (t = f1(), f2(t));
    assert_eq!(val, 2);
    std::process::exit(match val == 2 {
        true => 0,
        false => 1,
    });
}