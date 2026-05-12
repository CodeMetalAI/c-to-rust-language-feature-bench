fn main() {
    let mut c: u8;
    let i: i32 = 100;

    let l: i64 = {
        c = i as u8;
        c as i64
    };

    if c != (i as u8) {
        std::process::exit(1);
    }
    if l != (c as i64) {
        std::process::exit(2);
    }
    if l != (i as u8 as i64) {
        std::process::exit(3);
    }
}