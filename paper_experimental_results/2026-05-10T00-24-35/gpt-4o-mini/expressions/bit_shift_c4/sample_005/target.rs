fn main() -> i32 {
    let x: u32 = 1;

    let y = x << 3;
    if y != 8 {
        return 1;
    }

    if (0u32 << 5) != 0 {
        return 2;
    }

    if (2u32 << 2) != 8 {
        return 3;
    }

    0
}