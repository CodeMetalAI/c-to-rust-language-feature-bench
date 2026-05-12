fn main() -> i32 {
    let x = 1 - 1; // Equivalent to OBJ_LIKE
    let y = func_like(3); // Equivalent to FUNC_LIKE(3)

    if x != 0 {
        return 1;
    }
    if y != 3 {
        return 2;
    }

    return 0;
}

fn func_like(a: i32) -> i32 {
    a // Equivalent to FUNC_LIKE(a)
}