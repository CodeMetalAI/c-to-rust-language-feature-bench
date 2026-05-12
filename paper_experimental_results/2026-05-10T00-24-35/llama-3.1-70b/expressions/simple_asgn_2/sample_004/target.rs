fn main() {
    let mut c: u8;
    let i: i32 = 100;
    let l: i64;

    l = {
        c = i as u8;
        c as i64
    };

    if c!= (i as u8) {
        return;
    }
    if l!= (c as i64) {
        std::process::exit(2);
    }
    if l!= (i as u8 as i64) {
        std::process::exit(3);
    }

    std::process::exit(0);
}