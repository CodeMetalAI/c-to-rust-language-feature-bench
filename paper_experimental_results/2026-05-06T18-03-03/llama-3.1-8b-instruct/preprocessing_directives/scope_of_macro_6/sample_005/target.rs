fn main() {
    let x = if cfg!(feature = "negative") {
        0
    } else {
        1 - 1
    };
    let y = if cfg!(feature = "negative") {
        3
    } else {
        3
    };

    if x != 0 {
        std::process::exit(1);
    }
    if y != 3 {
        std::process::exit(2);
    }

    std::process::exit(0);
}