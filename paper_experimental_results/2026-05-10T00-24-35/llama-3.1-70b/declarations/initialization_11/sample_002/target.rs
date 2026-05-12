fn main() {
    let w = [[1, 0, 0], [2, 0, 0]];

    if w.len()!= 2 {
        return;
    }

    if w[0][0]!= 1 {
        return;
    }
    if w[0][1]!= 0 {
        return;
    }
    if w[0][2]!= 0 {
        return;
    }

    if w[1][0]!= 2 {
        return;
    }
    if w[1][1]!= 0 {
        return;
    }
    if w[1][2]!= 0 {
        return;
    }
}