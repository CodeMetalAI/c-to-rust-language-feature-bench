fn main() {
    let c: i8;
    let i: i32;
    let l: i64;

    i = 100;

    l = i as i8;
    c = l as i8;

    if c != i as i8 {
        std::process::exit(1);
    }
    if l != c as i64 {
        std::process::exit(2);
    }
    if l != i as i8 as i64 {
        std::process::exit(3);
    }

    std::process::exit(0);
}