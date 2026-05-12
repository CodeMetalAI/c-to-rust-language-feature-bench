struct W {
    a: [i32; 3],
    b: i32,
}

fn main() {
    let w = [W { a: [1, 0, 0], b: 2 }];

    if w.len() != 1 {
        return 1;
    }

    if w[0].a[0] != 1 {
        return 2;
    }
    if w[0].a[1] != 0 {
        return 3;
    }
    if w[0].a[2] != 0 {
        return 4;
    }
    if w[0].b != 2 {
        return 5;
    }

    if w[0].a[0] != 2 {
        return 6;
    }
    if w[0].a[1] != 0 {
        return 7;
    }
    if w[0].a[2] != 0 {
        return 8;
    }
    if w[0].b != 2 {
        return 9;
    }

    return 0;
}