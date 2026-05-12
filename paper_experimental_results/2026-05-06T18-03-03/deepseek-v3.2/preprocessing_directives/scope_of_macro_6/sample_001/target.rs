fn main() {
    const OBJ_LIKE: i32 = 1 - 1;
    const OBJ_LIKE_DUPLICATE: i32 = 1 - 1;

    fn func_like(a: i32) -> i32 {
        a
    }
    fn func_like_duplicate(a: i32) -> i32 {
        a
    }

    // The NEGATIVE conditional compilation block is omitted since it's not defined

    let x = OBJ_LIKE;
    let y = func_like(3);

    if x != 0 {
        std::process::exit(1);
    }
    if y != 3 {
        std::process::exit(2);
    }

    std::process::exit(0);
}