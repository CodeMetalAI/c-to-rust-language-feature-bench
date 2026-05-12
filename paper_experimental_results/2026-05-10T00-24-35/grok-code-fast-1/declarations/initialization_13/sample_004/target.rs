fn main() -> i32 {
    let x = 42i32;
    if x != 42 {
        return 1;
    }

    let y = 7u32;
    if y != 7 {
        return 2;
    }

    0
}