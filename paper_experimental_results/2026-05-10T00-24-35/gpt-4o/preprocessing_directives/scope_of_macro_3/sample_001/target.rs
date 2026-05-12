fn f(a: i32) -> i32 {
    a + 2
}

fn main() -> i32 {
    let v = f(10);
    let u = f(4); // Since (3, 4) evaluates to 4 in C
    let w = f(0);

    let i = [1, 23, 4, 5];
    let c = ["hello", ""];

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

    if c[0].as_bytes()[0] != b'h' {
        return 8;
    }
    if c[0].as_bytes()[1] != b'e' {
        return 9;
    }
    if c[0].as_bytes()[2] != b'l' {
        return 10;
    }
    if c[0].as_bytes()[3] != b'l' {
        return 11;
    }
    if c[0].as_bytes()[4] != b'o' {
        return 12;
    }
    if c[0].as_bytes()[5] != b'\0' {
        return 13;
    }

    if c[1].as_bytes()[0] != b'\0' {
        return 14;
    }

    0
}