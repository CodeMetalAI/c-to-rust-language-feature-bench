use std::process;

#[derive(Copy, Clone)]
struct W {
    a: [i32; 3],
    b: i32,
}

static W_ARR: [W; 2] = [
    W { a: [1, 0, 0], b: 0 },
    W { a: [2, 0, 0], b: 0 },
];

fn main() {
    if W_ARR.len() != 2 {
        process::exit(1);
    }

    if W_ARR[0].a[0] != 1 {
        process::exit(2);
    }
    if W_ARR[0].a[1] != 0 {
        process::exit(3);
    }
    if W_ARR[0].a[2] != 0 {
        process::exit(4);
    }
    if W_ARR[0].b != 0 {
        process::exit(5);
    }

    if W_ARR[1].a[0] != 2 {
        process::exit(6);
    }
    if W_ARR[1].a[1] != 0 {
        process::exit(7);
    }
    if W_ARR[1].a[2] != 0 {
        process::exit(8);
    }
    if W_ARR[1].b != 0 {
        process::exit(9);
    }

    process::exit(0);
}