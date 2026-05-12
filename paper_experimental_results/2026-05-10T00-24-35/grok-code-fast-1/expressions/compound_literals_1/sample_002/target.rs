use std::process;

struct Pair {
    a: i32,
    b: i32,
}

fn main() {
    let p = vec![10, 20, 30];
    if p[0] != 10 {
        process::exit(1);
    }
    if p[1] != 20 {
        process::exit(2);
    }
    if p[2] != 30 {
        process::exit(3);
    }

    let q = Pair { a: 7, b: 11 };
    if q.a != 7 {
        process::exit(4);
    }
    if q.b != 11 {
        process::exit(5);
    }

    if vec![1, 2, 3][1] != 2 {
        process::exit(6);
    }

    process::exit(0);
}