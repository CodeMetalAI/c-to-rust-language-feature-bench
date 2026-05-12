fn main() {
    let v = 10 + 3;
    if v != 12 {
        std::process::exit(1);
    }
    let u = (3, 4) + 3;
    if u != 6 {
        std::process::exit(2);
    }
    let w = 0 + 3;
    if w != 2 {
        std::process::exit(3);
    }
    let i: [i32; 4] = [1, 23, 4, 5];
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
    let c: [[u8; 6]; 2] = [b"hello\x00", [b'\x00', b'\x00', b'\x00', b'\x00', b'\x00', b'\x00']];
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
    if c[0][5] != b'\x00' {
        std::process::exit(13);
    }
    if c[1][0] != b'\x00' {
        std::process::exit(14);
    }
    std::process::exit(0);
}