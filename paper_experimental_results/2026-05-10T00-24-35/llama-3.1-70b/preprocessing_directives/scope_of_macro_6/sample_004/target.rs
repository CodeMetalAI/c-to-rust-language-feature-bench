fn main() {
    const OBJ_LIKE: i32 = 0;
    const OBJ_LIKE Shadowed: i32 = 0;

    let x = OBJ_LIKE;

    let y = func_like(3);

    if x!= 0 {
        return;
    }
    if y!= 3 {
        return;
    }
}

fn func_like(a: i32) -> i32 {
    a
}