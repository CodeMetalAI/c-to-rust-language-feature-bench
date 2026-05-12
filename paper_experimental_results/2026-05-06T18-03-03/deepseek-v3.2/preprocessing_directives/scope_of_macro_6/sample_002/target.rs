fn main() {
    const OBJ_LIKE: i32 = 1 - 1;
    const OBJ_LIKE_2: i32 = 1 - 1;

    fn func_like(a: i32) -> i32 {
        a
    }
    fn func_like_2(a: i32) -> i32 {
        a
    }

    #[cfg(NEGATIVE)]
    {
        const OBJ_LIKE_NEG: i32 = 0;
        const OBJ_LIKE_NEG_2: i32 = 1e - 1;
        fn func_like_neg(b: i32) -> i32 {
            a
        }
        fn func_like_neg_2(b: i32) -> i32 {
            b
        }
    }

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