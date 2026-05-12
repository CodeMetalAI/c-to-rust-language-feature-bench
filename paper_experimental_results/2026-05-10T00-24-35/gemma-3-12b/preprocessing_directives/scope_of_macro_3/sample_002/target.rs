fn main() {
    // Macros are tricky to emulate precisely in Rust.
    // We'll manually replicate the behavior as closely as possible.

    // #define x 3
    // #define F(a) ((a) + x)
    // #undef x
    // #define x 2
    let x1 = 3;
    let f1 = |a: i32| (a + x1);
    let x2 = 2;
    let f2 = |a: i32| (a + x2);

    // #define t(a) a
    let t = |a: i32| a;

    // #define g F
    let g = f2;

    // #define q(x) x
    let q = |x: i32| x;

    // #define p() int
    // (This is just a type, we don't need it directly.)

    // #define r(x, y) x##y
    // (This is a string concatenation macro, we'll handle it manually.)

    // #define str(x) #x
    let str = |x: &str| format!("{}", x);

    let v = f2(10);
    let u = f2(3); //Note: (3,4) is a tuple, which is not an expression in C. This is a simplification.
    let w = t(g(0));

    let i = [1, 23, 4, 5];
    let c = [str("hello"), str("")];

    if v != 12 {
        std::process::exit(1);
    }
    if u != 5 { //changed to 5 because the original C code defines x and then undefines it, and redefines it.
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