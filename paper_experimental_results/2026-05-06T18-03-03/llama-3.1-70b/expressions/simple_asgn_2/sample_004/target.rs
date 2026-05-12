fn main() {
    let mut c: u8;
    let mut i: i32;
    let mut l: i64;

    i = 100;

    c = i as u8;
    l = c as i64;

    if c!= (i as u8) {
        std::process::exit(1);
    }
    if l!= (c as i64) {
        std::process::exit(2);
    }
    if l!= (i as u8 as i64) {
        std::process::exit(3);
    }

    std::process::exit(0);
}