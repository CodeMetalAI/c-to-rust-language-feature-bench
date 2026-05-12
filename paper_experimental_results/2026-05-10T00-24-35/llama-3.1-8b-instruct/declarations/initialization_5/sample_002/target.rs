fn main() {
    let data = [1, 0, 0, 2];
    let w = [
        struct_ { data: [0; 3], b: 0 },
        struct_ { data: data, b: 0 },
    ];

    if w.len() != 2 {
        return 1;
    }

    if w[0].data[0] != 1 {
        return 2;
    }
    if w[0].data[1] != 0 {
        return 3;
    }
    if w[0].data[2] != 0 {
        return 4;
    }
    if w[0].b != 0 {
        return 5;
    }

    if w[1].data[0] != 2 {
        return 6;
    }
    if w[1].data[1] != 0 {
        return 7;
    }
    if w[1].data[2] != 0 {
        return 8;
    }
    if w[1].b != 0 {
        return 9;
    }

    return 0;
}

struct struct_ {
    data: [i32; 3],
    b: i32,
}