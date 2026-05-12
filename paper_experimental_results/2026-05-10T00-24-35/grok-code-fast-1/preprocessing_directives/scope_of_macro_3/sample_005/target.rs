fn main() {
    let v = 10 + 2; // F(10) = ((10) + x), x=2
    let u = (3, 4).1 + 2; // F((3,4)) = (4) + 2
    let w = 0 + 2; // t(g)(0) = g(0) = F(0) = 0 + 2

    let i: [i32; 4] = [1, 23, 4, 5]; // q(1), r(2,3), r(4,), r(,5)

    let c: [[u8; 6]; 2] = [
        *b"hello\0",
        [b'\0'; 6],
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