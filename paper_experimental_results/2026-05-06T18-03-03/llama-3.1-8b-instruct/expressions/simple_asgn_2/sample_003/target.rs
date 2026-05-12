fn main() {
    let mut c: char = '\0';
    let mut i: i32 = 100;
    let mut l: i64 = 0;

    l = (c = i) as i64;

    if c != i as u8 as char {
        std::process::exit(1);
    }
    if l != c as i64 {
        std::process::exit(2);
    }
    if l != i as i64 {
        std::process::exit(3);
    }

    std::process::exit(0);
}