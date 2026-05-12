fn chk(v: i16, e: i16) -> bool {
    v == e
}

fn main() {
    let q = [
        [[1, 0], [0, 0], [0, 0]],
        [[2, 3], [0, 0], [0, 0]],
        [[4, 5], [6, 0], [0, 0]],
        [[0, 0], [0, 0], [0, 0]],
    ];

    if !chk(q[0][0][0], 1) {
        return std::process::exit(1);
    }
    if !chk(q[0][0][1], 0) {
        return std::process::exit(2);
    }
    if !chk(q[0][1][0], 0) {
        return std::process::exit(3);
    }
    if !chk(q[0][2][1], 0) {
        return std::process::exit(4);
    }

    if !chk(q[1][0][0], 2) {
        return std::process::exit(5);
    }
    if !chk(q[1][0][1], 3) {
        return std::process::exit(6);
    }
    if !chk(q[1][1][0], 0) {
        return std::process::exit(7);
    }

    if !chk(q[2][0][0], 4) {
        return std::process::exit(8);
    }
    if !chk(q[2][0][1], 5) {
        return std::process::exit(9);
    }
    if !chk(q[2][1][0], 6) {
        return std::process::exit(10);
    }
    if !chk(q[2][1][1], 0) {
        return std::process::exit(11);
    }

    if !chk(q[3][0][0], 0) {
        return std::process::exit(12);
    }
    if !chk(q[3][2][1], 0) {
        return std::process::exit(13);
    }

    let p = [
        q[0][0][0], q[0][0][1], q[0][1][0], q[0][1][1], q[0][2][0], q[0][2][1],
        q[1][0][0], q[1][0][1], q[1][1][0], q[1][1][1], q[1][2][0], q[1][2][1],
        q[2][0][0], q[2][0][1], q[2][1][0], q[2][1][1], q[2][2][0], q[2][2][1],
        q[3][0][0], q[3][0][1], q[3][1][0], q[3][1][1], q[3][2][0], q[3][2][1],
    ];

    if !chk(p[0], 1) {
        return std::process::exit(14);
    }
    if !chk(p[6], 2) {
        return std::process::exit(15);
    }
    if !chk(p[7], 3) {
        return std::process::exit(16);
    }
    if !chk(p[12], 4) {
        return std::process::exit(17);
    }
    if !chk(p[13], 5) {
        return std::process::exit(18);
    }
    if !chk(p[14], 6) {
        return std::process::exit(19);
    }
}