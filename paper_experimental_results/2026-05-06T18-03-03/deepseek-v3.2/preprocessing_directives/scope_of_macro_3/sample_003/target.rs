fn main() {
    // Equivalent of macros:
    // #define x 3
    // #define F(a) ((a) + x)
    // #undef x
    // #define x 2
    // So F(a) expands to ((a) + 2) because x is redefined to 2 before usage.
    let v = 10 + 2; // F(10) -> ((10) + 2) = 12
    let u = (3, 4) + 2; // F((3, 4)) -> ((3, 4) + 2) = 6 (C comma operator yields 4, then 4+2=6)
    // #define t(a) a
    // #define g F
    // int w = t(g)(0); -> t(F)(0) -> F(0) -> ((0) + 2) = 2
    let w = 0 + 2;

    // #define p() int
    // #define q(x) x
    // #define r(x, y) x##y
    // p() i[q()] = {q(1), r(2, 3), r(4, ), r(, 5), r(, )};
    // -> int i[] = {1, 23, 4, 5, };
    // In Rust, we create an array with these values.
    let i: [i32; 5] = [1, 23, 4, 5, 0]; // r(, ) yields empty token -> 0

    // #define str(x) #x
    // char c[2][6] = {str(hello), str()};
    // -> c[0] = "hello", c[1] = ""
    // In Rust, we create a 2D array of chars (or string slices).
    let c0 = "hello";
    let c1 = "";
    // Check each character as in the C code.
    let c0_chars: Vec<char> = c0.chars().collect();
    let c1_chars: Vec<char> = c1.chars().collect();

    // Perform the same checks as the C program.
    if v != 12 {
        return std::process::exit(1);
    }
    if u != 6 {
        return std::process::exit(2);
    }
    if w != 2 {
        return std::process::exit(3);
    }

    if i[0] != 1 {
        return std::process::exit(4);
    }
    if i[1] != 23 {
        return std::process::exit(5);
    }
    if i[2] != 4 {
        return std::process::exit(6);
    }
    if i[3] != 5 {
        return std::process::exit(7);
    }
    if i[4] != 0 {
        return std::process::exit(15); // Added check for last element (r(, ))
    }

    if c0_chars[0] != 'h' {
        return std::process::exit(8);
    }
    if c0_chars[1] != 'e' {
        return std::process::exit(9);
    }
    if c0_chars[2] != 'l' {
        return std::process::exit(10);
    }
    if c0_chars[3] != 'l' {
        return std::process::exit(11);
    }
    if c0_chars[4] != 'o' {
        return std::process::exit(12);
    }
    if c0_chars.len() != 5 || c0_chars[5] != '\0' {
        // Rust strings don't have explicit null terminator in array, but we can check length.
        // C expects c[0][5] == '\0', so ensure string length is 5 (plus implicit null in C).
        // In Rust, "hello" is 5 chars, no null stored.
        return std::process::exit(13);
    }

    if c1_chars.len() != 0 || c1_chars[0] != '\0' {
        // c[1] is empty string, so first char is null terminator in C.
        // In Rust, empty string has no chars, but we simulate the check.
        return std::process::exit(14);
    }

    std::process::exit(0);
}