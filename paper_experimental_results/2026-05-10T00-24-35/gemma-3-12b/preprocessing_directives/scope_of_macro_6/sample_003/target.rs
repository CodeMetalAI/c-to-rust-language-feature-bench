fn main() {
    let obj_like = if cfg!(NEGATIVE) {
        0
    } else {
        1 - 1
    };

    let func_like = if cfg!(NEGATIVE) {
        3
    } else {
        let a = 3;
        a
    };

    if obj_like != 0 {
        std::process::exit(1);
    }
    if func_like != 3 {
        std::process::exit(2);
    }

    std::process::exit(0);
}