fn main() {
    // F(a) expands to ((a) + x), but x is redefined to 2 before use
    // So F(10) = 10 + 2 = 12
    let v: i32 = 10 + 2;
    
    // F((3, 4)) - the comma operator in C evaluates to 4, so F(4) = 4 + 2 = 6
    let u: i32 = 4 + 2;
    
    // t(g)(0) expands to g(0) which expands to F(0) = 0 + 2 = 2
    let w: i32 = 0 + 2;
    
    // q(1) = 1
    // r(2, 3) = 23 (token pasting)
    // r(4, ) = 4 (token pasting with empty)
    // r(, 5) = 5 (token pasting with empty)
    // r(, ) = nothing, but in array context this would be 0 or omitted
    // q() with no argument gives empty, so array size is unspecified - but based on initializers it's 5 elements
    let i: [i32; 5] = [1, 23, 4, 5, 0];
    
    // str(hello) = "hello"
    // str() = ""
    let c: [[u8; 6]; 2] = [*b"hello\0", [0, 0, 0, 0, 0, 0]];
    
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
    if c[0][5] != b'\0' {
        std::process::exit(13);
    }
    
    if c[1][0] != b'\0' {
        std::process::exit(14);
    }
    
    std::process::exit(0);
}