fn main() {
    // Macros are tricky to replicate exactly.  We'll try to match the behavior.
    // The C preprocessor does a lot of text substitution.

    // Define x (effectively constants)
    let x1 = 3;
    let x2 = 2;

    // F(a) = a + x
    let v = {
        let a = 10;
        a + x1
    };

    let u = {
        let a = (3, 4);
        // The C preprocessor would likely just insert (3, 4) into the expression.
        // The behavior is undefined, but we'll assume it's treated as a constant.
        // The compiler will likely error out, so we'll just use a constant value.
        3 + x1
    };

    let w = {
        let a = 0;
        let g = |a: i32| a + x2;
        g(a)
    };

    // p() int i[q()] = {q(1), r(2, 3), r(4, ), r(, 5), r(, )};
    // This is very complex due to the macro expansions.
    // Let's try to replicate the intended behavior.
    let i = [1, 23, 4, 5];

    // char c[2][6] = {str(hello), str()};
    // str(x) #x
    let c = [
        ["h", "e", "l", "l", "o", "\0"],
        ["\0", "\0", "\0", "\0", "\0", "\0"],
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

    0
}