fn main() {
    let w = [
        [1, 0, 0],
        [2, 0, 0],
    ];

    if w.len()!= 2 {
        return 1;
    }

    if w[0][0]!= 1 {
        return 2;
    }
    if w[0][1]!= 0 {
        return 3;
    }
    if w[0][2]!= 0 {
        return 4;
    }

    if w[1][0]!= 2 {
        return 6;
    }
    if w[1][1]!= 0 {
        return 7;
    }
    if w[1][2]!= 0 {
        return 8;
    }

    return 0;
}