fn chk(v: i16, e: i16) -> bool {
    v == e
}

fn main() {
    let q = [[1], [2, 3], [4, 5, 6]];
    if!chk(q[0][0], 1) {
        return 1;
    }
    if!chk(q[0][1], 0) {
        return 2;
    }
    if!chk(q[0][2], 0) {
        return 3;
    }
    if!chk(q[0][1][1], 0) {
        return 4;
    }

    if!chk(q[1][0], 2) {
        return 5;
    }
    if!chk(q[1][1], 3) {
        return 6;
    }
    if!chk(q[1][0][1], 0) {
        return 7;
    }

    if!chk(q[2][0], 4) {
        return 8;
    }
    if!chk(q[2][1], 5) {
        return 9;
    }
    if!chk(q[2][1][1], 0) {
        return 10;
    }
    if!chk(q[2][2], 6) {
        return 11;
    }

    if!chk(q[3][0], 0) {
        return 12;
    }
    if!chk(q[3][2], 0) {
        return 13;
    }

    let q_slice = &q[0][0];
    let q_ptr = q_slice.as_ptr() as *const i16;
    let p = unsafe { q_ptr.offset(0) };
    if!chk(*p, 1) {
        return 14;
    }
    if!chk(*p.offset(6), 2) {
        return 15;
    }
    if!chk(*p.offset(7), 3) {
        return 16;
    }
    if!chk(*p.offset(12), 4) {
        return 17;
    }
    if!chk(*p.offset(13), 5) {
        return 18;
    }
    if!chk(*p.offset(14), 6) {
        return 19;
    }
}