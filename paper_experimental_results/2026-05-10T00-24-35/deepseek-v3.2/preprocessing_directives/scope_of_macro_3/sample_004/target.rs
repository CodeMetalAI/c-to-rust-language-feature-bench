fn main() {
    let v = 10 + 2; // F(10) -> ((10) + x) where x is 2
    let u = (3, 4) + 2; // F((3,4)) -> (((3,4)) + x) -> 4 + 2 = 6
    let w = {
        fn f(a: i32) -> i32 {
            a + 2
        }
        f(0)
    }; // t(g)(0) -> F(0) -> ((0) + x) -> 0 + 2 = 2

    let i: [i32; 4] = [
        1,          // q(1) -> 1
        23,         // r(2,3) -> 23
        4,          // r(4,) -> 4
        5,          // r(,5) -> 5
    ];

    let c: [[char; 6]; 2] = [
        ['h', 'e', 'l', 'l', 'o', '\0'], // str(hello) -> "hello"
        ['\0', '\0', '\0', '\0', '\0', '\0'], // str() -> ""
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

    if c[0][0] != 'h' {
        std::process::exit(8);
    }
    if c[0][1] != 'e' {
        std::process::exit(9);
    }
    if c[0][2] != 'l' {
        std::process::exit(10);
    }
    if c[0][3] != 'l' {
        std::process::exit(11);
    }
    if c[0][4] != 'o' {
        std::process::exit(12);
    }
    if c[0][5] != '\0' {
        std::process::exit(13);
    }

    if c[1][0] != '\0' {
        std::process::exit(14);
    }

    std::process::exit(0);
}