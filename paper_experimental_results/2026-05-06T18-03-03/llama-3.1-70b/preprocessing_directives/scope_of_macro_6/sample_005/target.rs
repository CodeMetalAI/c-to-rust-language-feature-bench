fn main() {
    const OBJ_LIKE: i32 = 0;
    let y = 3;

    if OBJ_LIKE!= 0 {
        std::process::exit(1);
    }
    if y!= 3 {
        std::process::exit(2);
    }

    std::process::exit(0);
}