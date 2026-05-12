fn main() -> i32 {
    let x: u32 = 16;

    let y = x >> 2;
    if y != 4 {
        return 1;
    }

    if (0u32 >> 5) != 0 {
        return 2;
    }

    if (8u32 >> 1) != 4 {
        return 3;
    }

    0
}