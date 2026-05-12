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
        std::process::exit(1);
    }
    if !chk(q[0][0][1], 0) {
        std::process::exit(2);
    }
    if !chk(q[0][1][0], 0) {
        std::process::exit(3);
    }
    if !chk(q[0][2][1], 0) {
        std::process::exit(4);
    }

    if !chk(q[1][0][0], 2) {
        std::process::exit(5);
    }
    if !chk(q[1][0][1], 3) {
        std::process::exit(6);
    }
    if !chk(q[1][1][0], 0) {
        std::process::exit(7);
    }

    if !chk(q[2][0][0], 4) {
        std::process::exit(8);
    }
    if !chk(q[2][0][1], 5) {
        std::process::exit(9);
    }
    if !chk(q[2][1][0], 6) {
        std::process::exit(10);
    }
    if !chk(q[2][1][1], 0) {
        std::process::exit(11);
    }

    if !chk(q[3][0][0], 0) {
        std::process::exit(12);
    }
    if !chk(q[3][2][1], 0) {
        std::process::exit(13);
    }

    let flat: Vec<i16> = q
        .iter()
        .flat_map(|a| a.iter().flat_map(|b| b.iter().copied()))
        .collect();

    if !chk(flat[0], 1) {
        std::process::exit(14);
    }
    if !chk(flat[6], 2) {
        std::process::exit(15);
    }
    if !chk(flat[7], 3) {
        std::process::exit(16);
    }
    if !chk(flat[12], 4) {
        std::process::exit(17);
    }
    if !chk(flat[13], 5) {
        std::process::exit(18);
    }
    if !chk(flat[14], 6) {
        std::process::exit(19);
    }

    std::process::exit(0);
}