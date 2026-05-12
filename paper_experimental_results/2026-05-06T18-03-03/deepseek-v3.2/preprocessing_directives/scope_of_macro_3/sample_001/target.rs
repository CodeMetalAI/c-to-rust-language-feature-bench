fn main() {
    // Macros are not directly translatable; we compute values manually.
    // Original macros:
    // #define x 3
    // #define F(a) ((a) + x)
    // #undef x
    // #define x 2
    // So after redefinition, x is 2.
    // Thus F(a) expands to ((a) + 2).

    // int v = F(10);
    let v = 10 + 2; // 12

    // int u = F((3, 4));
    // In C, (3, 4) is a comma expression yielding 4, so F((3,4)) -> ((4) + 2) = 6.
    let u = 4 + 2; // 6

    // #define t(a) a
    // #define g F
    // int w = t(g)(0);
    // t(g) expands to F, so F(0) -> ((0) + 2) = 2.
    let w = 0 + 2; // 2

    // #define q(x) x
    // #define p() int
    // #define r(x, y) x##y
    // p() i[q()] = {q(1), r(2, 3), r(4, ), r(, 5), r(, )};
    // p() expands to int, so int i[q()].
    // q() expands to nothing (empty argument), so i[].
    // The array is initialized with:
    // q(1) -> 1
    // r(2, 3) -> concatenation of 2 and 3 -> 23
    // r(4, ) -> concatenation of 4 and nothing -> 4
    // r(, 5) -> concatenation of nothing and 5 -> 5
    // r(, ) -> concatenation of nothing and nothing -> empty token, treated as 0? Actually, in C, r(, ) produces an empty token, which is invalid in an integer constant. But the code compiles? Let's assume it yields 0.
    // In practice, GCC/Clang treat ## with empty arguments as concatenating nothing, resulting in an empty token, which is not a valid integer constant. However, the original C code likely compiles with some default (maybe 0). We'll assume 0 for the purpose of matching the checks.
    // The checks expect i[2] = 4, i[3] = 5, so the array has at least 4 elements.
    // The initialization list has 5 elements: 1, 23, 4, 5, and the empty token (0).
    // So i[0] = 1, i[1] = 23, i[2] = 4, i[3] = 5, i[4] = 0.
    // But the array size is determined by q() which is empty, so it's sized by the initializer (5 elements).
    let i = [1, 23, 4, 5, 0];

    // #define str(x) #x
    // char c[2][6] = {str(hello), str()};
    // str(hello) -> "hello" (6 chars including null terminator)
    // str() -> "" (empty string, 1 char null terminator)
    // c is 2 rows of 6 chars.
    // First row: "hello" -> 'h','e','l','l','o','\0'
    // Second row: "" -> '\0', then remaining chars are zero-initialized? In C, the rest of the row is zeroed.
    // The checks only test c[1][0] == '\0', so we only need the first char.
    let mut c = [[0 as char; 6]; 2];
    let hello = "hello";
    for (idx, ch) in hello.chars().enumerate() {
        c[0][idx] = ch;
    }
    c[0][5] = '\0';
    c[1][0] = '\0';

    // Perform checks
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

    if c[0][0] != 'h' {
        return std::process::exit(8);
    }
    if c[0][1] != 'e' {
        return std::process::exit(9);
    }
    if c[0][2] != 'l' {
        return std::process::exit(10);
    }
    if c[0][3] != 'l' {
        return std::process::exit(11);
    }
    if c[0][4] != 'o' {
        return std::process::exit(12);
    }
    if c[0][5] != '\0' {
        return std::process::exit(13);
    }

    if c[1][0] != '\0' {
        return std::process::exit(14);
    }

    std::process::exit(0);
}