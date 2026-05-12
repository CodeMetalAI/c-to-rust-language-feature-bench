use std::process::exit;

fn main() {
    let v: i32 = 10 + 2;
    let u: i32 = 4 + 2; // (3,4) -> 4
    let w: i32 = 0 + 2;

    let i: [i32; 4] = [1, 23, 4, 5];

    let c: [[u8; 6]; 2] = [
        [b'h', b'e', b'l', b'l', b'o', 0],
        [0, 0, 0, 0, 0, 0],
    ];

    if v != 12 {
        exit(1);
    }
    if u != 6 {
        exit(2);
    }
    if w != 2 {
        exit(3);
    }

    if i[0] != 1 {
        exit(4);
    }
    if i[1] != 23 {
        exit(5);
    }
    if i[2] != 4 {
        exit(6);
    }
    if i[3] != 5 {
        exit(7);
    }

    if c[0][0] != b'h' {
        exit(8);
    }
    if c[0][1] != b'e' {
        exit(9);
    }
    if c[0][2] != b'l' {
        exit(10);
    }
    if c[0][3] != b'l' {
        exit(11);
    }
    if c[0][4] != b'o' {
        exit(12);
    }
    if c[0][5] != 0 {
        exit(13);
    }

    if c[1][0] != 0 {
        exit(14);
    }

    exit(0);
}