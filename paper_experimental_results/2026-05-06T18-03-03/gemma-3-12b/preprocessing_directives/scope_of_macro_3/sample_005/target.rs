fn main() {
    // Macros are tricky to replicate exactly.  We'll try to match the behavior.
    // Note: C preprocessor macros are text substitutions, not code.

    // #define x 3
    // #define F(a) ((a) + x)
    // #undef x
    // #define x 2
    let x = 3;
    let v = (10 + x) - 1; // F(10) = 10 + 3 = 13, then x is undefined, then x = 2.  So 10 + 3 - 1 = 12
    let x = 2;
    let u = (3 + x); // F((3, 4)) = (3, 4) + 2.  The comma operator evaluates to 3.  So 3 + 2 = 5.  However, the C preprocessor will evaluate (3,4) to 3.
    let x = 2;
    let w = (0 + x); // t(g)(0) = g(0) = F(0) = 0 + 2 = 2

    // #define t(a) a
    // #define g F
    // #define q(x) x
    // #define p() int
    // #define r(x, y) x##y
    // #define str(x) #x

    let i = [1, 23, 4, 5];
    let c = [
        ["h", "e", "l", "l", "o", "\0"],
        ["\0", "\0", "\0", "\0", "\0", "\0"],
    ];

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

    0
}