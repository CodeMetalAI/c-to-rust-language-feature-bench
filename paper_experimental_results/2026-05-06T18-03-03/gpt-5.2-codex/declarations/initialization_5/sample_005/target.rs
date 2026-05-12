use std::process::exit;

#[derive(Copy, Clone)]
struct W {
    a: [i32; 3],
    b: i32,
}

const W_ARRAY: [W; 2] = [
    W { a: [1, 0, 0], b: 0 },
    W { a: [2, 0, 0], b: 0 },
];

fn main() {
    if W_ARRAY.len() != 2 {
        exit(1);
    }

    if W_ARRAY[0].a[0] != 1 {
        exit(2);
    }
    if W_ARRAY[0].a[1] != 0 {
        exit(3);
    }
    if W_ARRAY[0].a[2] != 0 {
        exit(4);
    }
    if W_ARRAY[0].b != 0 {
        exit(5);
    }

    if W_ARRAY[1].a[0] != 2 {
        exit(6);
    }
    if W_ARRAY[1].a[1] != 0 {
        exit(7);
    }
    if W_ARRAY[1].a[2] != 0 {
        exit(8);
    }
    if W_ARRAY[1].b != 0 {
        exit(9);
    }

    exit(0);
}