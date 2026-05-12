fn main() {
    let obj_like = 0;
    let func_like = 3;

    if obj_like != 0 {
        std::process::exit(1);
    }
    if func_like != 3 {
        std::process::exit(2);
    }

    std::process::exit(0);
}