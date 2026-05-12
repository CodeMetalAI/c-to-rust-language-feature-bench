fn main() {
    let x1: i32 = 7;
    let x2 = "hi";

    let fmt = "x1 = %d, x2 = %s";
    let inc = format!("vers2");
    let a = "hello";
    let b = "hello, world";

    if x1 != 7 {
        return 1;
    }
    if x2.chars().next().unwrap() != 'h' {
        return 2;
    }
    if x2.chars().nth(1).unwrap() != 'i' {
        return 3;
    }
    if x2.chars().nth(2).unwrap() != '\0' {
        return 4;
    }

    if fmt.len() != 15 {
        return 5;
    }
    if fmt.as_bytes()[0] != b'x' {
        return 6;
    }
    if fmt.as_bytes()[1] != b'1' {
        return 7;
    }
    if fmt.as_bytes()[2] != b'=' {
        return 8;
    }
    if fmt.as_bytes()[3] != b' ' {
        return 9;
    }
    if fmt.as_bytes()[4] != b'%' {
        return 10;
    }
    if fmt.as_bytes()[5] != b'd' {
        return 11;
    }
    if fmt.as_bytes()[6] != b',' {
        return 12;
    }
    if fmt.as_bytes()[7] != b' ' {
        return 13;
    }
    if fmt.as_bytes()[8] != b'x' {
        return 14;
    }
    if fmt.as_bytes()[9] != b'2' {
        return 15;
    }
    if fmt.as_bytes()[10] != b'=' {
        return 16;
    }
    if fmt.as_bytes()[11] != b' ' {
        return 17;
    }
    if fmt.as_bytes()[12] != b'%' {
        return 18;
    }
    if fmt.as_bytes()[13] != b's' {
        return 19;
    }
    if fmt.as_bytes()[14] != b'\0' {
        return 20;
    }

    if inc.len() != 8 {
        return 21;
    }
    if inc.as_bytes()[0] != b'v' {
        return 22;
    }
    if inc.as_bytes()[1] != b'e' {
        return 23;
    }
    if inc.as_bytes()[2] != b'r' {
        return 24;
    }
    if inc.as_bytes()[3] != b's' {
        return 25;
    }
    if inc.as_bytes()[4] != b'2' {
        return 26;
    }
    if inc.as_bytes()[5] != b'.' {
        return 27;
    }
    if inc.as_bytes()[6] != b'h' {
        return 28;
    }
    if inc.as_bytes()[7] != b'\0' {
        return 29;
    }

    if a.len() != 5 {
        return 30;
    }
    if a.as_bytes()[0] != b'h' {
        return 31;
    }
    if a.as_bytes()[1] != b'e' {
        return 32;
    }
    if a.as_bytes()[2] != b'l' {
        return 33;
    }
    if a.as_bytes()[3] != b'l' {
        return 34;
    }
    if a.as_bytes()[4] != b'o' {
        return 35;
    }

    if b.len() != 13 {
        return 36;
    }
    if b.as_bytes()[0] != b'h' {
        return 37;
    }
    if b.as_bytes()[1] != b'e' {
        return 38;
    }
    if b.as_bytes()[2] != b'l' {
        return 39;
    }
    if b.as_bytes()[3] != b'l' {
        return 40;
    }
    if b.as_bytes()[4] != b'o' {
        return 41;
    }
    if b.as_bytes()[5] != b',' {
        return 42;
    }
    if b.as_bytes()[6] != b' ' {
        return 43;
    }
    if b.as_bytes()[7] != b'w' {
        return 44;
    }
    if b.as_bytes()[8] != b'o' {
        return 45;
    }
    if b.as_bytes()[9] != b'r' {
        return 46;
    }
    if b.as_bytes()[10] != b'l' {
        return 47;
    }
    if b.as_bytes()[11] != b'd' {
        return 48;
    }
    if b.as_bytes()[12] != b'\0' {
        return 49;
    }

    return 0;
}