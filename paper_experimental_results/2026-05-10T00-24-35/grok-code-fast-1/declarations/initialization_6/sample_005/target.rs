use std::process;

fn chk(v: i16, e: i16) -> bool {
    v == e
}

fn main() {
    let q: [i16; 24] = [1, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    if !chk(q[0], 1) {
        process::exit(1);
    }
    if !chk(q[1], 0) {
        process::exit(2);
    }
    if !chk(q[2], 0) {
        process::exit(3);
    }
    if !chk(q[5], 0) {
        process::exit(4);
    }

    if !chk(q[6], 2) {
        process::exit(5);
    }
    if !chk(q[7], 3) {
        process::exit(6);
    }
    if !chk(q[8], 0) {
        process::exit(7);
    }

    if !chk(q[12], 4) {
        process::exit(8);
    }
    if !chk(q[13], 5) {
        process::exit(9);
    }
    if !chk(q[14], 6) {
        process::exit(10);
    }
    if !chk(q[15], 0) {
        process::exit(11);
    }

    if !chk(q[18], 0) {
        process::exit(12);
    }
    if !chk(q[23], 0) {
        process::exit(13);
    }

    {
        let p_idx = 0;
        if !chk(q[p_idx], 1) {
            process::exit(14);
        }
        if !chk(q[p_idx + 6], 2) {
            process::exit(15);
        }
        if !chk(q[p_idx + 7], 3) {
            process::exit(16);
        }
        if !chk(q[p_idx + 12], 4) {
            process::exit(17);
        }
        if !chk(q[p_idx + 13], 5) {
            process::exit(18);
        }
        if !chk(q[p_idx + 14], 6) {
            process::exit(19);
        }
    }

    process::exit(0);
}