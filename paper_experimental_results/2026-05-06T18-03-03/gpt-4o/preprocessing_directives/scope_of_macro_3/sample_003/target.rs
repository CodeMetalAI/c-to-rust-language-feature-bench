fn main() {
    const X: i32 = 2;

    fn f(a: i32) -> i32 {
        a + X
    }

    let v = f(10);
    let u = f(4); // (3, 4) evaluates to 4 in C/C++ due to the comma operator
    let w = f(0);

    let i = [1, 23, 4, 5];
    let c = ["hello", ""];

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

    if c[0].chars().nth(0) != Some('h') {
        std::process::exit(8);
    }
    if c[0].chars().nth(1) != Some('e') {
        std::process::exit(9);
    }
    if c[0].chars().nth(2) != Some('l') {
        std::process::exit(10);
    }
    if c[0].chars().nth(3) != Some('l') {
        std::process::exit(11);
    }
    if c[0].chars().nth(4) != Some('o') {
        std::process::exit(12);
    }
    if c[0].chars().nth(5) != Some('\0') {
        std::process::exit(13);
    }

    if c[1].chars().nth(0) != Some('\0') {
        std::process::exit(14);
    }

    std::process::exit(0);
}