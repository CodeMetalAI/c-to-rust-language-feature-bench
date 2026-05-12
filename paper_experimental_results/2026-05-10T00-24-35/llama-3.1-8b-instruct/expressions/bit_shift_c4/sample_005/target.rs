fn main() -> i32 {
    let x = 1u32;

    let y = x << 3;
    if y != 8u32 {
        return 1;
    }

    if 0u32 << 5 != 0u32 {
        return 2;
    }

    if 2u32 << 2 != 8u32 {
        return 3;
    }

    0
}