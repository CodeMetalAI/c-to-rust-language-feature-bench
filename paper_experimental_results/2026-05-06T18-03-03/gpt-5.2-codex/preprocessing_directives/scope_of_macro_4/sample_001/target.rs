use std::process::exit;

fn main() {
    let x1: i32 = 7;
    let x2: [u8; 3] = *b"hi\0";

    let fmt: [u8; 15] = *b"x1= %d, x2= %s\0";
    let inc: [u8; 8] = *b"vers2.h\0";
    let a: [u8; 6] = *b"hello\0";
    let b: [u8; 13] = *b"hello, world\0";

    if x1 != 7 {
        exit(1);
    }
    if x2[0] != b'h' {
        exit(2);
    }
    if x2[1] != b'i' {
        exit(3);
    }
    if x2[2] != b'\0' {
        exit(4);
    }

    if fmt.len() != 15 {
        exit(5);
    }
    if fmt[0] != b'x' {
        exit(6);
    }
    if fmt[1] != b'1' {
        exit(7);
    }
    if fmt[2] != b'=' {
        exit(8);
    }
    if fmt[3] != b' ' {
        exit(9);
    }
    if fmt[4] != b'%' {
        exit(10);
    }
    if fmt[5] != b'd' {
        exit(11);
    }
    if fmt[6] != b',' {
        exit(12);
    }
    if fmt[7] != b' ' {
        exit(13);
    }
    if fmt[8] != b'x' {
        exit(14);
    }
    if fmt[9] != b'2' {
        exit(15);
    }
    if fmt[10] != b'=' {
        exit(16);
    }
    if fmt[11] != b' ' {
        exit(17);
    }
    if fmt[12] != b'%' {
        exit(18);
    }
    if fmt[13] != b's' {
        exit(19);
    }
    if fmt[14] != b'\0' {
        exit(20);
    }

    if inc.len() != 8 {
        exit(21);
    }
    if inc[0] != b'v' {
        exit(22);
    }
    if inc[1] != b'e' {
        exit(23);
    }
    if inc[2] != b'r' {
        exit(24);
    }
    if inc[3] != b's' {
        exit(25);
    }
    if inc[4] != b'2' {
        exit(26);
    }
    if inc[5] != b'.' {
        exit(27);
    }
    if inc[6] != b'h' {
        exit(28);
    }
    if inc[7] != b'\0' {
        exit(29);
    }

    if a.len() != 6 {
        exit(30);
    }
    if a[0] != b'h' {
        exit(31);
    }
    if a[1] != b'e' {
        exit(32);
    }
    if a[2] != b'l' {
        exit(33);
    }
    if a[3] != b'l' {
        exit(34);
    }
    if a[4] != b'o' {
        exit(35);
    }
    if a[5] != b'\0' {
        exit(36);
    }

    if b.len() != 13 {
        exit(37);
    }
    if b[0] != b'h' {
        exit(38);
    }
    if b[1] != b'e' {
        exit(39);
    }
    if b[2] != b'l' {
        exit(40);
    }
    if b[3] != b'l' {
        exit(41);
    }
    if b[4] != b'o' {
        exit(42);
    }
    if b[5] != b',' {
        exit(43);
    }
    if b[6] != b' ' {
        exit(44);
    }
    if b[7] != b'w' {
        exit(45);
    }
    if b[8] != b'o' {
        exit(46);
    }
    if b[9] != b'r' {
        exit(47);
    }
    if b[10] != b'l' {
        exit(48);
    }
    if b[11] != b'd' {
        exit(49);
    }
    if b[12] != b'\0' {
        exit(50);
    }

    exit(0);
}