fn main() {
    let x1: i32 = 7;
    let x2: [u8; 3] = *b"hi\0";

    let fmt: [u8; 15] = *b"x1= %d, x2= %s\0";
    let inc: [u8; 8] = *b"vers2.h\0";
    let a: [u8; 6] = *b"hello\0";
    let b: [u8; 13] = *b"hello, world\0";

    let code = if x1 != 7 {
        1
    } else if x2[0] != b'h' {
        2
    } else if x2[1] != b'i' {
        3
    } else if x2[2] != 0 {
        4
    } else if fmt.len() != 15 {
        5
    } else if fmt[0] != b'x' {
        6
    } else if fmt[1] != b'1' {
        7
    } else if fmt[2] != b'=' {
        8
    } else if fmt[3] != b' ' {
        9
    } else if fmt[4] != b'%' {
        10
    } else if fmt[5] != b'd' {
        11
    } else if fmt[6] != b',' {
        12
    } else if fmt[7] != b' ' {
        13
    } else if fmt[8] != b'x' {
        14
    } else if fmt[9] != b'2' {
        15
    } else if fmt[10] != b'=' {
        16
    } else if fmt[11] != b' ' {
        17
    } else if fmt[12] != b'%' {
        18
    } else if fmt[13] != b's' {
        19
    } else if fmt[14] != 0 {
        20
    } else if inc.len() != 8 {
        21
    } else if inc[0] != b'v' {
        22
    } else if inc[1] != b'e' {
        23
    } else if inc[2] != b'r' {
        24
    } else if inc[3] != b's' {
        25
    } else if inc[4] != b'2' {
        26
    } else if inc[5] != b'.' {
        27
    } else if inc[6] != b'h' {
        28
    } else if inc[7] != 0 {
        29
    } else if a.len() != 6 {
        30
    } else if a[0] != b'h' {
        31
    } else if a[1] != b'e' {
        32
    } else if a[2] != b'l' {
        33
    } else if a[3] != b'l' {
        34
    } else if a[4] != b'o' {
        35
    } else if a[5] != 0 {
        36
    } else if b.len() != 13 {
        37
    } else if b[0] != b'h' {
        38
    } else if b[1] != b'e' {
        39
    } else if b[2] != b'l' {
        40
    } else if b[3] != b'l' {
        41
    } else if b[4] != b'o' {
        42
    } else if b[5] != b',' {
        43
    } else if b[6] != b' ' {
        44
    } else if b[7] != b'w' {
        45
    } else if b[8] != b'o' {
        46
    } else if b[9] != b'r' {
        47
    } else if b[10] != b'l' {
        48
    } else if b[11] != b'd' {
        49
    } else if b[12] != 0 {
        50
    } else {
        0
    };

    std::process::exit(code);
}