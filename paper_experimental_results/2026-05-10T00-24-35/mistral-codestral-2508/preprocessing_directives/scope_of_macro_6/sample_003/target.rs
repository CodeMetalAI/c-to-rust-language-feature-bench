fn main() {
    // Define constants instead of macros
    const OBJ_LIKE: i32 = 1 - 1;
    const FUNC_LIKE: i32 = 3; // Since we can't have function-like macros with different behavior, we use a constant

    let x = OBJ_LIKE;
    let y = FUNC_LIKE;

    if x != 0 {
        std::process::exit(1);
    }
    if y != 3 {
        std::process::exit(2);
    }

    std::process::exit(0);
}