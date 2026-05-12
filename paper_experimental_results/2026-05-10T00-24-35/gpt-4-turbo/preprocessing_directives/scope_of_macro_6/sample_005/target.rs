fn main() {
    let x = 0;  // Equivalent to OBJ_LIKE after macro expansion
    let y = 3;  // Equivalent to FUNC_LIKE(3) after macro expansion

    if x != 0 {
        std::process::exit(1);
    }
    if y != 3 {
        std::process::exit(2);
    }
}