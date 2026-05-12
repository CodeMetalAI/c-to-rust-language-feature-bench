fn main() {
    const X1: i32 = 7;
    let x2: [u8; 3] = *b"hi\0";

    // dbgfmt(1, 2) expands to: "x" "1" "= %d, x" "2" "= %s" → "x1= %d, x2= %s"
    let fmt: [u8; 15] = *b"x1= %d, x2= %s\0";

    // xstr(INCFILE(2).h) → xstr(vers2.h) → str(vers2.h) → "vers2.h"
    let inc: [u8; 8] = *b"vers2.h\0";

    // glue(HIGH, LOW) → HIGHLOW → "hello"
    let a: [u8; 6] = *b"hello\0";

    // xglue(HIGH, LOW) → glue(HIGH, LOW) → HIGHLOW → "hello" and then LOW → ", world"
    // Actually, the C macro expands HIGHLOW first, then LOW, resulting in "hello" ", world"
    let b: [u8; 13] = *b"hello, world\0";

    if X1 != 7 {
        std::process::exit(1);
    }
    if x2[0] != b'h' {
        std::process::exit(2);
    }
    if x2[1] != b'i' {
        std::process::exit(3);
    }
    if x2[2] != b'\0' {
        std::process::exit(4);
    }

    if fmt.len() != 15 {
        std::process::exit(5);
    }
    if fmt[0] != b'x' {
        std::process::exit(6);
    }
    if fmt[1] != b'1' {
        std::process::exit(7);
    }
    if fmt[2] != b'=' {
        std::process::exit(8);
    }
    if fmt[3] != b' ' {
        std::process::exit(9);
    }
    if fmt[4] != b'%' {
        std::process::exit(10);
    }
    if fmt[5] != b'd' {
        std::process::exit(11);
    }
    if fmt[6] != b',' {
        std::process::exit(12);
    }
    if fmt[7] != b' ' {
        std::process::exit(13);
    }
    if fmt[8] != b'x' {
        std::process::exit(14);
    }
    if fmt[9] != b'2' {
        std::process::exit(15);
    }
    if fmt[10] != b'=' {
        std::process::exit(16);
    }
    if fmt[11] != b' ' {
        std::process::exit(17);
    }
    if fmt[12] != b'%' {
        std::process::exit(18);
    }
    if fmt[13] != b's' {
        std::process::exit(19);
    }
    if fmt[14] != b'\0' {
        std::process::exit(20);
    }

    if inc.len() != 8 {
        std::process::exit(21);
    }
    if inc[0] != b'v' {
        std::process::exit(22);
    }
    if inc[1] != b'e' {
        std::process::exit(23);
    }
    if inc[2] != b'r' {
        std::process::exit(24);
    }
    if inc[3] != b's' {
        std::process::exit(25);
    }
    if inc[4] != b'2' {
        std::process::exit(26);
    }
    if inc[5] != b'.' {
        std::process::exit(27);
    }
    if inc[6] != b'h' {
        std::process::exit(28);
    }
    if inc[7] != b'\0' {
        std::process::exit(29);
    }

    if a.len() != 6 {
        std::process::exit(30);
    }
    if a[0] != b'h' {
        std::process::exit(31);
    }
    if a[1] != b'e' {
        std::process::exit(32);
    }
    if a[2] != b'l' {
        std::process::exit(33);
    }
    if a[3] != b'l' {
        std::process::exit(34);
    }
    if a[4] != b'o' {
        std::process::exit(35);
    }
    if a[5] != b'\0' {
        std::process::exit(36);
    }

    if b.len() != 13 {
        std::process::exit(37);
    }
    if b[0] != b'h' {
        std::process::exit(38);
    }
    if b[1] != b'e' {
        std::process::exit(39);
    }
    if b[2] != b'l' {
        std::process::exit(40);
    }
    if b[3] != b'l' {
        std::process::exit(41);
    }
    if b[4] != b'o' {
        std::process::exit(42);
    }
    if b[5] != b',' {
        std::process::exit(43);
    }
    if b[6] != b' ' {
        std::process::exit(44);
    }
    if b[7] != b'w' {
        std::process::exit(45);
    }
    if b[8] != b'o' {
        std::process::exit(46);
    }
    if b[9] != b'r' {
        std::process::exit(47);
    }
    if b[10] != b'l' {
        std::process::exit(48);
    }
    if b[11] != b'd' {
        std::process::exit(49);
    }
    if b[12] != b'\0' {
        std::process::exit(50);
    }

    std::process::exit(0);
}