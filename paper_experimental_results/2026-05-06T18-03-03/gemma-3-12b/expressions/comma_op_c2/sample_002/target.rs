fn f1() -> i32 {
    1
}

fn f2(t: i32) -> i32 {
    t + 1
}

fn main() {
    let mut t = 10;
    let val = {
        t = f1();
        f2(t)
    };
    if val == 2 {
        0
    } else {
        1
    }
}