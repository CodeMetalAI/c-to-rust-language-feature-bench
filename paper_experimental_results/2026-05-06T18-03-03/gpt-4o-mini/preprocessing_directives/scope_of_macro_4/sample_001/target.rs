const HIGHLOW: &str = "hello";
const LOW: &str = concat!(LOW, ", world");
const fn dbgfmt(s: usize, t: usize) -> &'static str {
    "x1= %d, x2= %s"
}

const x1: i32 = 7;
const x2: &str = "hi";

const fmt: &str = dbgfmt(1, 2);
const inc: &str = "vers2.h";
const a: &str = concat!(HIGHLOW, LOW);
const b: &str = concat!(HIGHLOW, LOW);

fn main() -> i32 {
    if x1 != 7 {
        return 1;
    }
    if x2.chars().nth(0).unwrap() != 'h' {
        return 2;
    }
    if x2.chars().nth(1).unwrap() != 'i' {
        return 3;
    }
    if x2.chars().nth(2).is_some() {
        return 4;
    }

    if fmt.len() != 15 {
        return 5;
    }
    if fmt.chars().nth(0).unwrap() != 'x' {
        return 6;
    }
    if fmt.chars().nth(1).unwrap() != '1' {
        return 7;
    }
    if fmt.chars().nth(2).unwrap() != '=' {
        return 8;
    }
    if fmt.chars().nth(3).unwrap() != ' ' {
        return 9;
    }
    if fmt.chars().nth(4).unwrap() != '%' {
        return 10;
    }
    if fmt.chars().nth(5).unwrap() != 'd' {
        return 11;
    }
    if fmt.chars().nth(6).unwrap() != ',' {
        return 12;
    }
    if fmt.chars().nth(7).unwrap() != ' ' {
        return 13;
    }
    if fmt.chars().nth(8).unwrap() != 'x' {
        return 14;
    }
    if fmt.chars().nth(9).unwrap() != '2' {
        return 15;
    }
    if fmt.chars().nth(10).unwrap() != '=' {
        return 16;
    }
    if fmt.chars().nth(11).unwrap() != ' ' {
        return 17;
    }
    if fmt.chars().nth(12).unwrap() != '%' {
        return 18;
    }
    if fmt.chars().nth(13).unwrap() != 's' {
        return 19;
    }
    if fmt.chars().nth(14).is_some() {
        return 20;
    }

    if inc.len() != 8 {
        return 21;
    }
    if inc.chars().nth(0).unwrap() != 'v' {
        return 22;
    }
    if inc.chars().nth(1).unwrap() != 'e' {
        return 23;
    }
    if inc.chars().nth(2).unwrap() != 'r' {
        return 24;
    }
    if inc.chars().nth(3).unwrap() != 's' {
        return 25;
    }
    if inc.chars().nth(4).unwrap() != '2' {
        return 26;
    }
    if inc.chars().nth(5).unwrap() != '.' {
        return 27;
    }
    if inc.chars().nth(6).unwrap() != 'h' {
        return 28;
    }
    if inc.chars().nth(7).is_some() {
        return 29;
    }

    if a.len() != 6 {
        return 30;
    }
    if a.chars().nth(0).unwrap() != 'h' {
        return 31;
    }
    if a.chars().nth(1).unwrap() != 'e' {
        return 32;
    }
    if a.chars().nth(2).unwrap() != 'l' {
        return 33;
    }
    if a.chars().nth(3).unwrap() != 'l' {
        return 34;
    }
    if a.chars().nth(4).unwrap() != 'o' {
        return 35;
    }
    if a.chars().nth(5).is_some() {
        return 36;
    }

    if b.len() != 13 {
        return 37;
    }
    if b.chars().nth(0).unwrap() != 'h' {
        return 38;
    }
    if b.chars().nth(1).unwrap() != 'e' {
        return 39;
    }
    if b.chars().nth(2).unwrap() != 'l' {
        return 40;
    }
    if b.chars().nth(3).unwrap() != 'l' {
        return 41;
    }
    if b.chars().nth(4).unwrap() != 'o' {
        return 42;
    }
    if b.chars().nth(5).unwrap() != ',' {
        return 43;
    }
    if b.chars().nth(6).unwrap() != ' ' {
        return 44;
    }
    if b.chars().nth(7).unwrap() != 'w' {
        return 45;
    }
    if b.chars().nth(8).unwrap() != 'o' {
        return 46;
    }
    if b.chars().nth(9).unwrap() != 'r' {
        return 47;
    }
    if b.chars().nth(10).unwrap() != 'l' {
        return 48;
    }
    if b.chars().nth(11).unwrap() != 'd' {
        return 49;
    }
    if b.chars().nth(12).is_some() {
        return 50;
    }

    0
}