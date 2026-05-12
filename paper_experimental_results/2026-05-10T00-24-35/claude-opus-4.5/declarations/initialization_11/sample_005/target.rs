struct W {
    a: [i32; 3],
    b: i32,
}

static W: [W; 2] = [
    W { a: [1, 0, 0], b: 0 },
    W { a: [2, 0, 0], b: 0 },
];

fn main() {
    if W.len() != 2 {
        std::process::exit(1);
    }

    if W[0].a[0] != 1 {
        std::process::exit(2);
    }
    if W[0].a[1] != 0 {
        std::process::exit(3);
    }
    if W[0].a[2] != 0 {
        std::process::exit(4);
    }
    if W[0].b != 0 {
        std::process::exit(5);
    }

    if W[1].a[0] != 2 {
        std::process::exit(6);
    }
    if W[1].a[1] != 0 {
        std::process::exit(7);
    }
    if W[1].a[2] != 0 {
        std::process::exit(8);
    }
    if W[1].b != 0 {
        std::process::exit(9);
    }

    std::process::exit(0);
}