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

    if !chk(q[0][0][0], 1) { return println!("{}", 1); }
    if !chk(q[0][0][1], 0) { return println!("{}", 2); }
    if !chk(q[0][1][0], 0) { return println!("{}", 3); }
    if !chk(q[0][2][1], 0) { return println!("{}", 4); }

    if !chk(q[1][0][0], 2) { return println!("{}", 5); }
    if !chk(q[1][0][1], 3) { return println!("{}", 6); }
    if !chk(q[1][1][0], 0) { return println!("{}", 7); }

    if !chk(q[2][0][0], 4) { return println!("{}", 8); }
    if !chk(q[2][0][1], 5) { return println!("{}", 9); }
    if !chk(q[2][1][0], 6) { return println!("{}", 10); }
    if !chk(q[2][1][1], 0) { return println!("{}", 11); }

    if !chk(q[3][0][0], 0) { return println!("{}", 12); }
    if !chk(q[3][2][1], 0) { return println!("{}", 13); }

    let p = &q[0][0][0];
    if !chk(p, &q[0][0][0]) { return println!("{}", 14); }
    if !chk(&q[1][0][0], 2) { return println!("{}", 15); }
    if !chk(&q[1][0][1], 3) { return println!("{}", 16); }
    if !chk(&q[2][0][0], 4) { return println!("{}", 17); }
    if !chk(&q[2][0][1], 5) { return println!("{}", 18); }
    if !chk(&q[2][1][0], 6) { return println!("{}", 19); }
}