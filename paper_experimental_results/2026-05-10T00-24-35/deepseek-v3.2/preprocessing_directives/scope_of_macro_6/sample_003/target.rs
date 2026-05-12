fn main() {
    const OBJ_LIKE: i32 = 1 - 1;
    const _OBJ_LIKE2: i32 = OBJ_LIKE;

    fn func_like(a: i32) -> i32 {
        a
    }
    fn _func_like2(a: i32) -> i32 {
        func_like(a)
    }

    // NEGATIVE is not defined, so skip those blocks

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