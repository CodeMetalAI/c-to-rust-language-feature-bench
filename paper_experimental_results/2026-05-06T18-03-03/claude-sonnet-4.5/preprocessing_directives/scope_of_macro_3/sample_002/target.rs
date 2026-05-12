fn main() {
    let v: i32 = 10 + 3;
    let u: i32 = 4 + 3;
    let w: i32 = 0 + 3;

    let i: [i32; 5] = [1, 23, 4, 5, 0];
    let c: [[u8; 6]; 2] = [
        [b'h', b'e', b'l', b'l', b'o', b'\0'],
        [b'\0', b'\0', b'\0', b'\0', b'\0', b'\0'],
    ];

    if v != 12 {
        std::process::exit(1);
    }
    if u != 6 {
        std::process::exit(2);
    }
    if w != 2 {
        std::process::exit(3);
    }

    if i[0] != 1 {
        std::process::exit(4);
    }
    if i[1] != 23 {
        std::process::exit(5);
    }
    if i[2] != 4 {
        std::process::exit(6);
    }
    if i[3] != 5 {
        std::process::exit(7);
    }

    if c[0][0] != b'h' {
        std::process::exit(8);
    }
    if c[0][1] != b'e' {
        std::process::exit(9);
    }
    if c[0][2] != b'l' {
        std::process::exit(10);
    }
    if c[0][3] != b'l' {
        std::process::exit(11);
    }
    if c[0][4] != b'o' {
        std::process::exit(12);
    }
    if c[0][5] != b'\0' {
        std::process::exit(13);
    }

    if c[1][0] != b'\0' {
        std::process::exit(14);
    }

    std::process::exit(0);
}