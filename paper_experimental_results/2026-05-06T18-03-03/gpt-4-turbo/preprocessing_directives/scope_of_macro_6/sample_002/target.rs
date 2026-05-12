fn main() {
    let x = 0; // Equivalent to OBJ_LIKE which is (1 - 1)
    let y = 3; // Equivalent to FUNC_LIKE(3) which is just (3)

    if x != 0 {
        std::process::exit(1);
    }
    if y != 3 {
        std::process::exit(2);
    }
}