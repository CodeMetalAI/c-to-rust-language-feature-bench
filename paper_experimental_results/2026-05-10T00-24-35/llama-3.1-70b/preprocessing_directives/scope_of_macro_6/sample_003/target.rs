fn main() {
    const OBJ_LIKE: i32 = 0;
    let y = |a| a;
    let x = OBJ_LIKE;
    let y = y(3);

    if x!= 0 {
        return;
    }
    if y!= 3 {
        std::process::exit(2);
    }
    std::process::exit(0);
}