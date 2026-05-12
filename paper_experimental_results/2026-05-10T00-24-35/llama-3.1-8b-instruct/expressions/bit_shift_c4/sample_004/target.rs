fn main() -> i32 {
    let x: u32 = 1;

    let y: u32 = x << 3;
    if y != 8 {
        return 1;
    }

    if 0 << 5 != 0 {
        return 2;
    }

    if 2 << 2 != 8 {
        return 3;
    }

    0
}