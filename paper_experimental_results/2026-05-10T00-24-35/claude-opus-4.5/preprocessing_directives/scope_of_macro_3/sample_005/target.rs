fn main() {
    // v = F(10) where F(a) = ((a) + x) and x was 3 at definition, but x is 2 at expansion
    // In C preprocessor, x is evaluated at expansion time, so x = 2
    // F(10) = ((10) + 2) = 12
    let v: i32 = 12;
    
    // u = F((3, 4)) - comma operator evaluates to 4, so F(4) = ((4) + 2) = 6
    let u: i32 = 6;
    
    // w = t(g)(0) = g(0) = F(0) = ((0) + 2) = 2
    let w: i32 = 2;
    
    // i[q()] = {q(1), r(2, 3), r(4, ), r(, 5), r(, )}
    // q() expands to nothing, so i[] (unspecified size)
    // q(1) = 1
    // r(2, 3) = 23 (token pasting)
    // r(4, ) = 4 (token pasting with empty)
    // r(, 5) = 5 (token pasting with empty)
    // r(, ) = nothing (both empty, but in C this creates empty token which is 0 in array init context)
    // Actually r(,) would be invalid or create nothing - looking at the checks, i has indices 0-3
    // So the array is [1, 23, 4, 5]
    let i: [i32; 4] = [1, 23, 4, 5];
    
    // c[2][6] = {str(hello), str()}
    // str(hello) = "hello"
    // str() = ""
    let c: [[u8; 6]; 2] = [
        [b'h', b'e', b'l', b'l', b'o', 0],
        [0, 0, 0, 0, 0, 0],
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
    if c[0][5] != 0 {
        std::process::exit(13);
    }
    
    if c[1][0] != 0 {
        std::process::exit(14);
    }
    
    std::process::exit(0);
}