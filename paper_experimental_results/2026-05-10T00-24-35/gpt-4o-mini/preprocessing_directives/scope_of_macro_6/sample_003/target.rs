fn main() -> i32 {
    let x = 1 - 1; // equivalent to OBJ_LIKE
    let y = 3;     // equivalent to FUNC_LIKE(3)

    if x != 0 {
        return 1;
    }
    if y != 3 {
        return 2;
    }

    return 0;
}