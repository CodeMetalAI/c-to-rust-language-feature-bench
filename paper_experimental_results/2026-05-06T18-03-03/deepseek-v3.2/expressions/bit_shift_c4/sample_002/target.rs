fn main() {
    let x: u32 = 1u;

    let y: u32 = x << 3;
    if y != 8u {
        return;
    }

    if (0u << 5) != 0u {
        return;
    }

    if (2u << 2) != 8u {
        return;
    }
}