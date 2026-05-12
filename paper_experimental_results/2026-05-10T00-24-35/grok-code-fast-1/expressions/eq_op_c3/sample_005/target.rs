fn to_i32(b: bool) -> i32 {
    if b { 1 } else { 0 }
}

fn main() {
    let mut a: i32;
    let mut b: i32;
    let mut c: i32;
    let mut d: i32;

    a = 1;
    b = 2;
    c = 3;
    d = 4;
    let left_cmp = a < b;
    let eq_part = to_i32(left_cmp) == c;
    let right_cmp = to_i32(eq_part) < d;
    if to_i32(right_cmp) != 1 {
        std::process::exit(1);
    }

    a = 2;
    b = 1;
    c = 4;
    d = 3;
    let left_cmp = a < b;
    let eq_part = to_i32(left_cmp) == c;
    let right_cmp = to_i32(eq_part) < d;
    if to_i32(right_cmp) != 1 {
        std::process::exit(2);
    }

    a = 1;
    b = 2;
    c = 4;
    d = 3;
    let left_cmp = a < b;
    let eq_part = to_i32(left_cmp) == c;
    let right_cmp = to_i32(eq_part) < d;
    if to_i32(right_cmp) != 0 {
        std::process::exit(3);
    }

    a = 2;
    b = 1;
    c = 3;
    d = 4;
    let left_cmp = a < b;
    let eq_part = to_i32(left_cmp) == c;
    let right_cmp = to_i32(eq_part) < d;
    if to_i32(right_cmp) != 0 {
        std::process::exit(4);
    }

    a = 1;
    b = 2;
    c = 3;
    d = 4;
    let expr = (a < b) == (c < d);
    if to_i32(expr) != 1 {
        std::process::exit(5);
    }

    std::process::exit(0);
}