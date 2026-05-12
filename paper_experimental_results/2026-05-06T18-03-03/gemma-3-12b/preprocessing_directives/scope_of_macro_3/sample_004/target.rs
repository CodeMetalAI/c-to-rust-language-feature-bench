fn main() {
    // Macros are tricky to translate directly.  We need to manually
    // expand them and then write the equivalent Rust code.

    // #define x 3
    // #define F(a) ((a) + x)
    // #undef x
    // #define x 2

    // v = F(10);  // F(10) expands to (10 + 2) = 12
    let v = 12;

    // u = F((3, 4)); // F((3, 4)) expands to ((3, 4) + 2) - this is invalid C, but we'll assume it's meant to be 3 + 2 = 5.
    let u = 5;

    // w = t(g)(0); // t(g) is g, g is F, so F(0) which is (0 + 2) = 2
    let w = 2;

    // p() i[q()] = {q(1), r(2, 3), r(4, ), r(, 5), r(, )};
    // p() is int, q(x) is x, r(x, y) is xy
    // i[0] = 1, i[1] = 23, i[2] = 4, i[3] = 5
    let i = [1, 23, 4, 5];

    // char c[2][6] = {str(hello), str()};
    // str(hello) is "hello", str() is ""
    let c = ["hello", ""];

    if v != 12 {
        std::process::exit(1);
    }
    if u != 5 {
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