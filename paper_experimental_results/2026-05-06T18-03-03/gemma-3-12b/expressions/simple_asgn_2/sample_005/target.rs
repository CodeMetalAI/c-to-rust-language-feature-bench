fn main() {
    let mut c: char = ' ';
    let mut i: i32 = 0;
    let mut l: i64 = 0;

    i = 100;

    l = (c = i as u8) as i64;

    if c != i as char {
        return 1;
    }
    if l != c as i64 {
        return 2;
    }
    if l != (i as u8 as i64) {
        return 3;
    }

    0
}