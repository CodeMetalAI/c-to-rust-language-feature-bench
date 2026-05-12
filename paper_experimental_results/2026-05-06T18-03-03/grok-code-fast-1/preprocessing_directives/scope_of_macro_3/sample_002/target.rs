use std::process;

fn main() {
    let v: i32 = 10 + 2;
    let u: i32 = 4 + 2;
    let w: i32 = 0 + 2;
    let i: [i32; 5] = [1, 23, 4, 5, 0];
    let c: [[u8; 6]; 2] = [b"hello\0", b"\0\0\0\0\0\0"];

    if v != 12 {
        process::exit(1);
    }
    if u != 6 {
        process::exit(2);
    }
    if w != 2 {
        process::exit(3);
    }
    if i[0] != 1 {
        process::exit(4);
    }
    if i[1] != 23 {
        process::exit(5);
    }
    if i[2] != 4 {
        process::exit(6);
    }
    if i[3] != 5 {
        process::exit(7);
    }
    if c[0][0] != b'h' {
        process::exit(8);
    }
    if c[0][1] != b'e' {
        process::exit(9);
    }
    if c[0][2] != b'l' {
        process::exit(10);
    }
    if c[0][3] != b'l' {
        process::exit(11);
    }
    if c[0][4] != b'o' {
        process::exit(12);
    }
    if c[0][5] != b'\0' {
        process::exit(13);
    }
    if c[1][0] != b'\0' {
        process::exit(14);
    }
    process::exit(0);
}