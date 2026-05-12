fn main() {
    const OBJ_LIKE: i32 = 0;
    const OBJ_LIKE2: i32 = 0; // Rust does not allow redefinition of constants

    let x = OBJ_LIKE;
    let y = 3; // Rust does not have macros like C, so we use a simple assignment

    if x!= 0 {
        std::process::exit(1);
    }
    if y!= 3 {
        std::process::exit(2);
    }

    std::process::exit(0);
}