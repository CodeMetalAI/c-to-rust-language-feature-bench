fn main() {
    let q: [i16; 24] = [
        1, 0, 0, 0, 0, 0,
        2, 3, 0, 0, 0, 0,
        4, 5, 6, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];

    let chk = |v: i16, e: i16| v == e;

    if !chk(q[0], 1) {
        std::process::exit(1);
    }
    if !chk(q[1], 0) {
        std::process::exit(2);
    }
    if !chk(q[2], 0) {
        std::process::exit(3);
    }
    if !chk(q[5], 0) {
        std::process::exit(4);
    }
    if !chk(q[6], 2) {
        std::process::exit(5);
    }
    if !chk(q[7], 3) {
        std::process::exit(6);
    }
    if !chk(q[8], 0) {
        std::process::exit(7);
    }
    if !chk(q[12], 4) {
        std::process::exit(8);
    }
    if !chk(q[13], 5) {
        std::process::exit(9);
    }
    if !chk(q[14], 6) {
        std::process::exit(10);
    }
    if !chk(q[15], 0) {
        std::process::exit(11);
    }
    if !chk(q[18], 0) {
        std::process::exit(12);
    }
    if !chk(q[23], 0) {
        std::process::exit(13);
    }

    if !chk(q[0], 1) {
        std::process::exit(14);
    }
    if !chk(q[6], 2) {
        std::process::exit(15);
    }
    if !chk(q[7], 3) {
        std::process::exit(16);
    }
    if !chk(q[12], 4) {
        std::process::exit(17);
    }
    if !chk(q[13], 5) {
        std::process::exit(18);
    }
    if !chk(q[14], 6) {
        std::process::exit(19);
    }

    std::process::exit(0);
}