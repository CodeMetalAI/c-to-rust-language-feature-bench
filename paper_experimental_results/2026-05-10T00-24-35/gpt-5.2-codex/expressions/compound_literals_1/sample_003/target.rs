use std::process::exit;

struct Pair {
    a: i32,
    b: i32,
}

fn main() {
    let arr = [10, 20, 30];
    let p = &arr;

    if p[0] != 10 {
        exit(1);
    }
    if p[1] != 20 {
        exit(2);
    }
    if p[2] != 30 {
        exit(3);
    }

    let q = Pair { a: 7, b: 11 };
    if q.a != 7 {
        exit(4);
    }
    if q.b != 11 {
        exit(5);
    }

    if [1, 2, 3][1] != 2 {
        exit(6);
    }
}