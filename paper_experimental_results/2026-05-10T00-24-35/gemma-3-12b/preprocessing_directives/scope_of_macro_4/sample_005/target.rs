fn main() {
    let x1 = 7;
    let x2 = "hi";

    let fmt = "x1= %d, x2= %s";
    let inc = "vers2.h";
    let a = "hello";
    let b = "hello, world";

    if x1 != 7 {
        return 1;
    }
    if x2.chars().nth(0) != Some('h') {
        return 2;
    }
    if x2.chars().nth(1) != Some('i') {
        return 3;
    }
    if x2.chars().nth(2) != Some('\0') {
        return 4;
    }

    if fmt.len() != 15 {
        return 5;
    }
    if fmt.chars().nth(0) != Some('x') {
        return 6;
    }
    if fmt.chars().nth(1) != Some('1') {
        return 7;
    }
    if fmt.chars().nth(2) != Some('=') {
        return 8;
    }
    if fmt.chars().nth(3) != Some(' ') {
        return 9;
    }
    if fmt.chars().nth(4) != Some('%') {
        return 10;
    }
    if fmt.chars().nth(5) != Some('d') {
        return 11;
    }
    if fmt.chars().nth(6) != Some(',') {
        return 12;
    }
    if fmt.chars().nth(7) != Some(' ') {
        return 13;
    }
    if fmt.chars().nth(8) != Some('x') {
        return 14;
    }
    if fmt.chars().nth(9) != Some('2') {
        return 15;
    }
    if fmt.chars().nth(10) != Some('=') {
        return 16;
    }
    if fmt.chars().nth(11) != Some(' ') {
        return 17;
    }
    if fmt.chars().nth(12) != Some('%') {
        return 18;
    }
    if fmt.chars().nth(13) != Some('s') {
        return 19;
    }
    if fmt.chars().nth(14) != Some('\0') {
        return 20;
    }

    if inc.len() != 8 {
        return 21;
    }
    if inc.chars().nth(0) != Some('v') {
        return 22;
    }
    if inc.chars().nth(1) != Some('e') {
        return 23;
    }
    if inc.chars().nth(2) != Some('r') {
        return 24;
    }
    if inc.chars().nth(3) != Some('s') {
        return 25;
    }
    if inc.chars().nth(4) != Some('2') {
        return 26;
    }
    if inc.chars().nth(5) != Some('.') {
        return 27;
    }
    if inc.chars().nth(6) != Some('h') {
        return 28;
    }
    if inc.chars().nth(7) != Some('\0') {
        return 29;
    }

    if a.len() != 5 {
        return 30;
    }
    if a.chars().nth(0) != Some('h') {
        return 31;
    }
    if a.chars().nth(1) != Some('e') {
        return 32;
    }
    if a.chars().nth(2) != Some('l') {
        return 33;
    }
    if a.chars().nth(3) != Some('l') {
        return 34;
    }
    if a.chars().nth(4) != Some('o') {
        return 35;
    }
    if a.chars().nth(5) != Some('\0') {
        return 36;
    }

    if b.len() != 12 {
        return 37;
    }
    if b.chars().nth(0) != Some('h') {
        return 38;
    }
    if b.chars().nth(1) != Some('e') {
        return 39;
    }
    if b.chars().nth(2) != Some('l') {
        return 40;
    }
    if b.chars().nth(3) != Some('l') {
        return 41;
    }
    if b.chars().nth(4) != Some('o') {
        return 42;
    }
    if b.chars().nth(5) != Some(',') {
        return 43;
    }
    if b.chars().nth(6) != Some(' ') {
        return 44;
    }
    if b.chars().nth(7) != Some('w') {
        return 45;
    }
    if b.chars().nth(8) != Some('o') {
        return 46;
    }
    if b.chars().nth(9) != Some('r') {
        return 47;
    }
    if b.chars().nth(10) != Some('l') {
        return 48;
    }
    if b.chars().nth(11) != Some('d') {
        return 49;
    }
    if b.chars().nth(12) != Some('\0') {
        return 50;
    }

    0
}