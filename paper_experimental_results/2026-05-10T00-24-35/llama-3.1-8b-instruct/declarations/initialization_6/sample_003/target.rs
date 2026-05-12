fn chk(v: i16, e: i16) -> bool {
    v == e
}

fn main() {
    let q: [[i16; 2]; 3; 4] = [
        [[1], [0], [0], [0]],
        [[2, 3], [0], [0], [0]],
        [[4, 5, 6], [0], [0], [0]],
    ];

    if!chk(q[0][0][0], 1) {
        println!("Exit code 1");
        return;
    }
    if!chk(q[0][0][1], 0) {
        println!("Exit code 2");
        return;
    }
    if!chk(q[0][1][0], 0) {
        println!("Exit code 3");
        return;
    }
    if!chk(q[0][2][1], 0) {
        println!("Exit code 4");
        return;
    }

    if!chk(q[1][0][0], 2) {
        println!("Exit code 5");
        return;
    }
    if!chk(q[1][0][1], 3) {
        println!("Exit code 6");
        return;
    }
    if!chk(q[1][1][0], 0) {
        println!("Exit code 7");
        return;
    }

    if!chk(q[2][0][0], 4) {
        println!("Exit code 8");
        return;
    }
    if!chk(q[2][0][1], 5) {
        println!("Exit code 9");
        return;
    }
    if!chk(q[2][1][0], 6) {
        println!("Exit code 10");
        return;
    }
    if!chk(q[2][1][1], 0) {
        println!("Exit code 11");
        return;
    }

    if!chk(q[3][0][0], 0) {
        println!("Exit code 12");
        return;
    }
    if!chk(q[3][2][1], 0) {
        println!("Exit code 13");
        return;
    }

    let p = &q[0][0][0];
    if!chk(p[0], 1) {
        println!("Exit code 14");
        return;
    }
    if!chk(p[6], 2) {
        println!("Exit code 15");
        return;
    }
    if!chk(p[7], 3) {
        println!("Exit code 16");
        return;
    }
    if!chk(p[12], 4) {
        println!("Exit code 17");
        return;
    }
    if!chk(p[13], 5) {
        println!("Exit code 18");
        return;
    }
    if!chk(p[14], 6) {
        println!("Exit code 19");
        return;
    }

    println!("Exit code 0");
}