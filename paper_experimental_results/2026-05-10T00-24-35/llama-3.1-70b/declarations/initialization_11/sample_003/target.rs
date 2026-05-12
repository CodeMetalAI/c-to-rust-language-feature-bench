fn main() {
    #[derive(Debug, PartialEq)]
    struct W {
        a: [i32; 3],
        b: i32,
    }

    let w = [W { a: [1, 0, 0], b: 0 }, W { a: [2, 0, 0], b: 0 }];

    if w.len() != 2 {
        std::process::exit(1);
    }

    if w[0].a[0] != 1 {
        std::process::exit(2);
    } else if w[0].a[1] != 0 {
        std::process::exit(3);
    } else if w[0].a[2] != 0 {
        std::process::exit(4);
    } else if w[0].b != 0 {
        std::process::exit(5);
    }

    if w[1].a[0] != 2 {
        std::process::exit(6);
    } else if w[1].a[1] != 0 {
        std::process::exit(7);
    } else if w[1].a[2] != 0 {
        std::process::exit(8);
    } else if w[1].b != 0 {
        std::process::exit(9);
    }
}