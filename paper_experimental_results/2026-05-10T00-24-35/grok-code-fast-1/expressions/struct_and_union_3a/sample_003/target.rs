fn main() {
    let mut type_: i32 = 0;
    let mut doublenode: f64 = 0.0;

    type_ = 1;
    doublenode = 3.14;

    if type_ != 1 {
        std::process::exit(1);
    }
    if type_ != 1 {
        std::process::exit(1);
    }
    if type_ != 1 {
        std::process::exit(2);
    }
    if doublenode != 3.14 {
        std::process::exit(3);
    }

    std::process::exit(0);
}