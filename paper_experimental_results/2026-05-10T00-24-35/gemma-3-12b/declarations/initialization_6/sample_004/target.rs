fn chk(v: i16, e: i16) -> bool {
    v == e
}

fn main() {
    let q: [[i16; 3]; 2] = [[1], [2, 3]];

    if !chk(q[0][0], 1) {
        std::process::exit(1);
    }
    if !chk(q[0][1], 0) {
        std::process::exit(2);
    }
    if !chk(q[1][0], 0) {
        std::process::exit(3);
    }

    if !chk(q[1][1], 2) {
        std::process::exit(5);
    }
    if !chk(q[1][2], 3) {
        std::process::exit(6);
    }

    if !chk(q[0][0], 4) {
        std::process::exit(8);
    }
    if !chk(q[0][1], 5) {
        std::process::exit(9);
    }
    if !chk(q[0][2], 6) {
        std::process::exit(10);
    }
    if !chk(q[0][3], 0) {
        std::process::exit(11);
    }


    std::process::exit(0);
}