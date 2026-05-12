use std::process;

#[derive(Clone, Copy)]
struct W {
    a: [i32; 3],
    b: i32,
}

fn main() {
    let w = [
        W { a: [1, 0, 0], b: 0 },
        W { a: [2, 0, 0], b: 0 },
    ];

    let code = if w.len() != 2 {
        1
    } else if w[0].a[0] != 1 {
        2
    } else if w[0].a[1] != 0 {
        3
    } else if w[0].a[2] != 0 {
        4
    } else if w[0].b != 0 {
        5
    } else if w[1].a[0] != 2 {
        6
    } else if w[1].a[1] != 0 {
        7
    } else if w[1].a[2] != 0 {
        8
    } else if w[1].b != 0 {
        9
    } else {
        0
    };

    process::exit(code);
}