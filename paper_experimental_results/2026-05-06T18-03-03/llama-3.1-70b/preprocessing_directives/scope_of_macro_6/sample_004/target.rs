fn main() {
    const OBJ_LIKE: i32 = 0;
    const OBJ_LIKE2: i32 = 0; // Renamed to avoid conflict

    let x = OBJ_LIKE;
    let y = 3; // Simulating the behavior of FUNC_LIKE(3)

    if x != 0 {
        std::process::exit(1);
    }
    if y != 3 {
        std::process::exit(2);
    }

    std::process::exit(0);
}