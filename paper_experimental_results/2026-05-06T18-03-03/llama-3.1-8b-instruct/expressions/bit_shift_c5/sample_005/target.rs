fn main() {
    let x: u32 = 16;
    let y = x >> 2;
    if y!= 4 {
        return 1;
    }

    if 0 >> 5!= 0 {
        return 2;
    }

    if 8 >> 1!= 4 {
        return 3;
    }

    return 0;
}