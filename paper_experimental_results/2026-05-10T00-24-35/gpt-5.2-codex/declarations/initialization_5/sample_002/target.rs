use std::mem::size_of;
use std::process::exit;

#[derive(Copy, Clone)]
struct WStruct {
    a: [i32; 3],
    b: i32,
}

static W: [WStruct; 2] = [
    WStruct { a: [1, 0, 0], b: 0 },
    WStruct { a: [2, 0, 0], b: 0 },
];

fn main() {
    if size_of::<[WStruct; 2]>() / size_of::<WStruct>() != 2 {
        exit(1);
    }

    if W[0].a[0] != 1 {
        exit(2);
    }
    if W[0].a[1] != 0 {
        exit(3);
    }
    if W[0].a[2] != 0 {
        exit(4);
    }
    if W[0].b != 0 {
        exit(5);
    }

    if W[1].a[0] != 2 {
        exit(6);
    }
    if W[1].a[1] != 0 {
        exit(7);
    }
    if W[1].a[2] != 0 {
        exit(8);
    }
    if W[1].b != 0 {
        exit(9);
    }

    exit(0);
}