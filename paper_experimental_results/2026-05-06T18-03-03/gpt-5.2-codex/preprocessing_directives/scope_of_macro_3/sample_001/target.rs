use std::process::exit;

const V: i32 = 12;
const U: i32 = 6;
const W: i32 = 2;

const I: [i32; 4] = [1, 23, 4, 5];
const C: [[u8; 6]; 2] = [*b"hello\0", [0; 6]];

fn main() {
    if V != 12 {
        exit(1);
    }
    if U != 6 {
        exit(2);
    }
    if W != 2 {
        exit(3);
    }

    if I[0] != 1 {
        exit(4);
    }
    if I[1] != 23 {
        exit(5);
    }
    if I[2] != 4 {
        exit(6);
    }
    if I[3] != 5 {
        exit(7);
    }

    if C[0][0] != b'h' {
        exit(8);
    }
    if C[0][1] != b'e' {
        exit(9);
    }
    if C[0][2] != b'l' {
        exit(10);
    }
    if C[0][3] != b'l' {
        exit(11);
    }
    if C[0][4] != b'o' {
        exit(12);
    }
    if C[0][5] != b'\0' {
        exit(13);
    }

    if C[1][0] != b'\0' {
        exit(14);
    }

    exit(0);
}