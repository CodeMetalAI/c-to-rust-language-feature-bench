fn chk(v: i16, e: i16) -> bool {
    v == e
}

fn main() {
    let q = [[1], [2, 3], [4, 5, 6]];
    let q_2d = [[q[0][0], q[0][1], q[0][2]], [q[1][0], q[1][1], q[1][2]], [q[2][0], q[2][1], q[2][2]]];
    let q_3d = [[[q_2d[0][0], q_2d[0][1], q_2d[0][2]], [q_2d[1][0], q_2d[1][1], q_2d[1][2]], [q_2d[2][0], q_2d[2][1], q_2d[2][2]]]];

    if !chk(q_3d[0][0][0], 1) {
        return;
    }
    if !chk(q_3d[0][0][1], 0) {
        return;
    }
    if !chk(q_3d[0][1][0], 0) {
        return;
    }
    if !chk(q_3d[0][2][1], 0) {
        return;
    }

    if !chk(q_3d[1][0][0], 2) {
        return;
    }
    if !chk(q_3d[1][0][1], 3) {
        return;
    }
    if !chk(q_3d[1][1][0], 0) {
        return;
    }

    if !chk(q_3d[2][0][0], 4) {
        return;
    }
    if !chk(q_3d[2][0][1], 5) {
        return;
    }
    if !chk(q_3d[2][1][0], 6) {
        return;
    }
    if !chk(q_3d[2][1][1], 0) {
        return;
    }

    if !chk(q_3d[3][0][0], 0) {
        return;
    }
    if !chk(q_3d[3][2][1], 0) {
        return;
    }

    let p = &q_3d[0][0][0];
    if !chk(p[0], 1) {
        return;
    }
    if !chk(p[6], 2) {
        return;
    }
    if !chk(p[7], 3) {
        return;
    }
    if !chk(p[12], 4) {
        return;
    }
    if !chk(p[13], 5) {
        return;
    }
    if !chk(p[14], 6) {
        return;
    }

    println!("All checks passed.");
}