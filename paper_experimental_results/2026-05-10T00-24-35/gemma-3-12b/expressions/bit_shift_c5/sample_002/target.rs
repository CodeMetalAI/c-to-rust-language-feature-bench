fn main() {
    let x: u32 = 16;

    let y = x >> 2;
    if y != 4 {
        return 1;
    }

    if (0u32 >> 5) != 0u32 {
        return 2;
    }

    if (8u32 >> 1) != 4u32 {
        return 3;
    }

    0
}