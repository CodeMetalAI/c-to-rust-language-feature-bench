fn main() {
    #[cfg(not(feature = "negative"))]
    const OBJ_LIKE: i32 = 1 - 1;

    #[cfg(not(feature = "negative"))]
    const fn func_like(a: i32) -> i32 {
        a
    }

    let x = OBJ_LIKE;
    let y = func_like(3);

    if x != 0 {
        std::process::exit(1);
    }
    if y != 3 {
        std::process::exit(2);
    }
}