use std::process::exit;

fn chk(v: i16, e: i16) -> bool {
    v == e
}

fn flat(q: &[[[i16; 2]; 3]; 4], idx: usize) -> i16 {
    let i = idx / 6;
    let rem = idx % 6;
    let j = rem / 2;
    let k = rem % 2;
    q[i][j][k]
}

fn main() {
    let q: [[[i16; 2]; 3]; 4] = [
        [[1, 0], [0, 0], [0, 0]],
        [[2, 3], [0, 0], [0, 0]],
        [[4, 5], [6, 0], [0, 0]],
        [[0, 0], [0, 0], [0, 0]],
    ];

    if !chk(q[0][0][0], 1) {
        exit(1);
    }
    if !chk(q[0][0][1], 0) {
        exit(2);
    }
    if !chk(q[0][1][0], 0) {
        exit(3);
    }
    if !chk(q[0][2][1], 0) {
        exit(4);
    }

    if !chk(q[1][0][0], 2) {
        exit(5);
    }
    if !chk(q[1][0][1], 3) {
        exit(6);
    }
    if !chk(q[1][1][0], 0) {
        exit(7);
    }

    if !chk(q[2][0][0], 4) {
        exit(8);
    }
    if !chk(q[2][0][1], 5) {
        exit(9);
    }
    if !chk(q[2][1][0], 6) {
        exit(10);
    }
    if !chk(q[2][1][1], 0) {
        exit(11);
    }

    if !chk(q[3][0][0], 0) {
        exit(12);
    }
    if !chk(q[3][2][1], 0) {
        exit(13);
    }

    if !chk(flat(&q, 0), 1) {
        exit(14);
    }
    if !chk(flat(&q, 6), 2) {
        exit(15);
    }
    if !chk(flat(&q, 7), 3) {
        exit(16);
    }
    if !chk(flat(&q, 12), 4) {
        exit(17);
    }
    if !chk(flat(&q, 13), 5) {
        exit(18);
    }
    if !chk(flat(&q, 14), 6) {
        exit(19);
    }

    exit(0);
}