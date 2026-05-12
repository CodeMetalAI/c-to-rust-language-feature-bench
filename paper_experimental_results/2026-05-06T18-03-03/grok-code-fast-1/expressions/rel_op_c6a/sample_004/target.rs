fn main() {
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    let mut c: i32 = 2;

    let temp = if a < b { 1 } else { 0 };
    if !(temp < c) {
        std::process::exit(1);
    }

    let temp2 = if a < b { 1 } else { 0 };
    if !(temp2 < c) {
        std::process::exit(2);
    }

    a = 3;
    b = 2;
    c = 2;

    let temp3 = if a < b { 1 } else { 0 };
    if !(temp3 < c) {
        std::process::exit(3);
    }

    std::process::exit(0);
}