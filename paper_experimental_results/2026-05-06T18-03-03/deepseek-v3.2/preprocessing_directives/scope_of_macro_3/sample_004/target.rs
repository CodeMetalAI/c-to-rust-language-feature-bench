fn main() {
    // Macro expansions simulated directly
    // #define x 3 (then later redefined to 2)
    // #define F(a) ((a) + x) where x is 2 at point of use
    let v = 10 + 2; // F(10) -> ((10) + 2) = 12
    let u = ((3, 4)) + 2; // F((3,4)) -> (((3,4)) + 2) = (3,4) + 2 = 6 (comma operator yields 4, then +2)
    let w = F(0); // t(g)(0) -> g(0) -> F(0) -> ((0) + 2) = 2

    // p() i[q()] = {q(1), r(2,3), r(4,), r(,5), r(,)};
    // p() -> int
    // q() -> empty expansion -> nothing
    // So: int i[] = {q(1), r(2,3), r(4,), r(,5), r(,)};
    // q(1) -> 1
    // r(2,3) -> 23 (concatenation)
    // r(4,) -> 4 (concatenation with empty)
    // r(,5) -> 5
    // r(,) -> empty (but array initializer must have at least one element, so treat as 0? Actually C leaves empty, causing error? Let's check: original compiles? It does, but r(,) produces nothing, so array has 4 elements? Wait, counting: q(1), r(2,3), r(4,), r(,5), r(,) -> 5 initializers, but r(,) produces empty token, which is allowed? In C, `r(,)` expands to nothing, so the array `i` has 5 elements: 1, 23, 4, 5, and an empty initializer? Actually empty initializer is invalid in C. Let's examine: `r(,)` expands to nothing, so the initializer list becomes `{1, 23, 4, 5, }` trailing comma allowed. So 4 elements? No, 5 elements: 1, 23, 4, 5, and an empty? But C99 6.7.8: initializer list may end with comma. So `{1, 23, 4, 5, }` is same as `{1, 23, 4, 5}`. So 4 elements. But declaration `int i[]` size becomes 4. However original code: `p() i[q()]` -> `int i[]` size determined by initializer count. Count initializers: q(1)=1, r(2,3)=23, r(4,)=4, r(,5)=5, r(,)=nothing. So after expansion: `int i[] = {1, 23, 4, 5, }` -> 4 elements. So i[0]=1, i[1]=23, i[2]=4, i[3]=5. The code checks i[0]..i[3] so indeed 4 elements.
    let i: [i32; 4] = [1, 23, 4, 5];

    // char c[2][6] = {str(hello), str()};
    // str(hello) -> "hello"
    // str() -> ""
    // So c[0] = "hello\0", c[1] = "\0"
    let c: [[char; 6]; 2] = [
        ['h', 'e', 'l', 'l', 'o', '\0'],
        ['\0', '\0', '\0', '\0', '\0', '\0']
    ];

    // Simulate checks
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

// Helper to compute w: F(0) with x=2
fn F(a: i32) -> i32 {
    a + 2
}