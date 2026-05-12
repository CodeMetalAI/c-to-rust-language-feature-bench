use std::process::exit;

fn chk(v: i16, e: i16) -> bool {
    v == e
}

fn main() {
    let mut q = [[[0i16; 2]; 3]; 4];
    q[0][0][0] = 1;
    q[1][0][0] = 2;
    q[1][0][1] = 3;
    q[2][0][0] = 4;
    q[2][0][1] = 5;
    q[2][1][0] = 6;

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

    {
        let mut flat = Vec::with_capacity(24);
        for a in 0..4 {
            for b in 0..3 {
                for c in 0..2 {
                    flat.push(q[a][b][c]);
                }
            }
        }

        if !chk(flat[0], 1) {
            exit(14);
        }
        if !chk(flat[6], 2) {
            exit(15);
        }
        if !chk(flat[7], 3) {
            exit(16);
        }
        if !chk(flat[12], 4) {
            exit(17);
        }
        if !chk(flat[13], 5) {
            exit(18);
        }
        if !chk(flat[14], 6) {
            exit(19);
        }
    }

    exit(0);
}