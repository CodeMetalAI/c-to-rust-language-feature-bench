fn main() {
    let mut c: char = '\0';
    let mut i: i32 = 100;
    let mut l: i64 = 0;

    l = c as i64 = i as i64;

    if c as i32 != i {
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