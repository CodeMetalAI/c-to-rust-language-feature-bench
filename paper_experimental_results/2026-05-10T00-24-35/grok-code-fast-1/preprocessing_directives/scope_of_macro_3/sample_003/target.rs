const V: i32 = 10 + 2;
const U: i32 = (3, 4).1 + 2;
const W: i32 = 0 + 2;

const I: [i32; 4] = [1, 23, 4, 5];

const C: [[u8; 6]; 2] = [
    [b'h', b'e', b'l', b'l', b'o', b'\0'],
    [b'\0', b'\0', b'\0', b'\0', b'\0', b'\0'],
];

fn main() {
    if V != 12 {
        std::process::exit(1);
    }
    if U != 6 {
        std::process::exit(2);
    }
    if W != 2 {
        std::process::exit(3);
    }

    if I[0] != 1 {
        std::process::exit(4);
    }
    if I[1] != 23 {
        std::process::exit(5);
    }
    if I[2] != 4 {
        std::process::exit(6);
    }
    if I[3] != 5 {
        std::process::exit(7);
    }

    if C[0][0] != b'h' {
        std::process::exit(8);
    }
    if C[0][1] != b'e' {
        std::process::exit(9);
    }
    if C[0][2] != b'l' {
        std::process::exit(10);
    }
    if C[0][3] != b'l' {
        std::process::exit(11);
    }
    if C[0][4] != b'o' {
        std::process::exit(12);
    }
    if C[0][5] != b'\0' {
        std::process::exit(13);
    }

    if C[1][0] != b'\0' {
        std::process::exit(14);
    }

    std::process::exit(0);
}