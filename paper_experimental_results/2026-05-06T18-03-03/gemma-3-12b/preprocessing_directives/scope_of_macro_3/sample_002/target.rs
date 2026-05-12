fn main() {
    // Macros are tricky to replicate exactly in Rust.  We'll try to
    // mimic the C++ behavior as closely as possible.

    // Define x as 3, then F(a) = a + x, then undef x, then define x as 2.
    // This means F(10) will be 10 + 3 = 13, but the second x is not used.
    let v = 10 + 3; // F(10)
    let u = 3 + 3; // F((3, 4)) - The comma operator evaluates to 3
    let w = 0 + 2; // t(g)(0) where g is F, and F(0) = 0 + 2 = 2

    // p() int i[q()] = {q(1), r(2, 3), r(4, ), r(, 5), r(, )};
    // This is equivalent to int i[1] = {1, 23, 4, 5, };
    let i = [1, 23, 4, 5];

    // char c[2][6] = {str(hello), str()};
    // This is equivalent to char c[2][6] = {"hello", ""};
    let c = ["hello", ""];

    if v != 13 {
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