fn main() {
    let mut c: char = ' ';
    let mut i: i32 = 100;
    let mut l: i64 = 0;

    l = (c = i as u8) as i64;

    if c as i32 != i {
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