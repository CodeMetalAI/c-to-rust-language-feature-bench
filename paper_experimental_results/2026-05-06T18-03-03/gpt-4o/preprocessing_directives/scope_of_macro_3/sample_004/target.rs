fn f(a: i32) -> i32 {
    a + 2
}

fn main() -> i32 {
    let v = f(10);
    let u = f(4); // (3, 4) evaluates to 4 in C/C++ due to comma operator
    let w = f(0);

    let i = [1, 23, 4, 5, 0];
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

    if c[0].chars().nth(0) != Some('h') {
        return 8;
    }
    if c[0].chars().nth(1) != Some('e') {
        return 9;
    }
    if c[0].chars().nth(2) != Some('l') {
        return 10;
    }
    if c[0].chars().nth(3) != Some('l') {
        return 11;
    }
    if c[0].chars().nth(4) != Some('o') {
        return 12;
    }
    if c[0].chars().nth(5) != Some('\0') {
        return 13;
    }

    if c[1].chars().nth(0) != Some('\0') {
        return 14;
    }

    0
}