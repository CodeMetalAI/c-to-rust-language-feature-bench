fn main() {
    const OBJ_LIKE: i32 = 1 - 1;
    
    let x = OBJ_LIKE;
    let y = 3;

    if x != 0 {
        std::process::exit(1);
    }
    if y != 3 {
        std::process::exit(2);
    }

    std::process::exit(0);
}