fn chk(v: i16, e: i16) -> bool {
    v == e
}

fn main() {
    let q: [[i16; 3]; 4] = [
        [1],
        [2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];

    if!chk(q[0][0][0], 1) {
        return;
    }
    if!chk(q[0][0][1], 0) {
        return;
    }
    if!chk(q[0][1][0], 0) {
        return;
    }
    if!chk(q[0][2][1], 0) {
        return;
    }

    if!chk(q[1][0][0], 2) {
        return;
    }
    if!chk(q[1][0][1], 3) {
        return;
    }
    if!chk(q[1][1][0], 0) {
        return;
    }

    if!chk(q[2][0][0], 4) {
        return;
    }
    if!chk(q[2][0][1], 5) {
        return;
    }
    if!chk(q[2][1][0], 6) {
        return;
    }
    if!chk(q[2][1][1], 0) {
        return;
    }

    if!chk(q[3][0][0], 0) {
        return;
    }
    if!chk(q[3][2][1], 0) {
        return;
    }

    let p: &[i16] = &q[0][0][..];
    if!chk(p[0], 1) {
        return;
    }
    if!chk(p[6], 2) {
        return;
    }
    if!chk(p[7], 3) {
        return;
    }
    if!chk(p[12], 4) {
        return;
    }
    if!chk(p[13], 5) {
        return;
    }
    if!chk(p[14], 6) {
        return;
    }

    println!("All assertions passed.");
}