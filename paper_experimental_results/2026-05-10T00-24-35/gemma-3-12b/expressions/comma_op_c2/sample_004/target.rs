fn f1() -> i32 {
    1
}

fn f2(t: i32) -> i32 {
    t + 1
}

fn main() {
    let mut t = 10;
    let val = {
        let temp = f1();
        let t_new = f2(t);
        t = t_new;
        t_new
    };

    if val == 2 {
        0
    } else {
        1
    }
}