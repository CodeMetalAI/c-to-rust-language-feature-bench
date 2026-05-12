fn main() {
    // Macros are tricky to translate directly. We'll manually replicate the behavior.

    // #define x 3
    // #define F(a) ((a) + x)
    // #undef x
    // #define x 2
    let x1 = 3;
    let f1 = |a: i32| (a + x1);
    let x2 = 2;
    let f2 = |a: i32| (a + x2);

    // int v = F(10);
    let v = f2(10);

    // int u = F((3, 4));
    // This is problematic because (3, 4) is a tuple, not an integer.  The original C code is undefined behavior.
    // We'll assume the intention was to add x2 to 3.
    let u = f2(3);

    // int w = t(g)(0);
    let t = |a: i32| a;
    let g = f2;
    let w = t(g)(0);

    // p() i[q()] = {q(1), r(2, 3), r(4, ), r(, 5), r(, )};
    // p() is ill-formed.  Assuming it's int.
    // q(x) is x
    // r(x, y) is x##y, where ## is a token pasting operator.
    let mut i = [0; 5];
    i[0] = 1;
    i[1] = 23;
    i[2] = 4;
    i[3] = 5;
    i[4] = 0; // Needed to initialize the array.

    // char c[2][6] = {str(hello), str()};
    // str(x) is #x, which is a string literal.
    let c = ["hello".as_bytes(), "".as_bytes()];

    if v != 12 {
        return 1;
    }
    if u != 6 {
        return 2;
    }
    if w != 2 {
        return 3;
    }

    if i[0] != 1 {
        return 4;
    }
    if i[1] != 23 {
        return 5;
    }
    if i[2] != 4 {
        return 6;
    }
    if i[3] != 5 {
        return 7;
    }

    if c[0][0] != b'h' {
        return 8;
    }
    if c[0][1] != b'e' {
        return 9;
    }
    if c[0][2] != b'l' {
        return 10;
    }
    if c[0][3] != b'l' {
        return 11;
    }
    if c[0][4] != b'o' {
        return 12;
    }
    if c[0][5] != 0 {
        return 13;
    }

    if c[1][0] != 0 {
        return 14;
    }

    0
}