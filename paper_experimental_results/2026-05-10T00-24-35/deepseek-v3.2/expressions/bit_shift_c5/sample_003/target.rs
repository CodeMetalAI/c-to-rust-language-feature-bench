fn main() {
    let x: u32 = 16u;

    let y = x >> 2;
    if y != 4u {
        return;
    }

    if (0u >> 5) != 0u {
        return;
    }

    if (8u >> 1) != 4u {
        return;
    }

    ();
}