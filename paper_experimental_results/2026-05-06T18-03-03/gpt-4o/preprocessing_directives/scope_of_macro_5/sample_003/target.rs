fn cat(a: u32, b: u32) -> u32 {
    a * 10 + b
}

fn cat3(a: u32, b: u32, c: u32) -> u32 {
    a * 100 + b * 10 + c
}

fn pickd(n: u32) -> u32 {
    n
}

fn vxyz(a: u32, b: u32, c: u32) -> u32 {
    cat3(pickd(a), pickd(b), pickd(c))
}

fn vyz(b: u32, c: u32) -> u32 {
    cat(pickd(b), pickd(c))
}

fn vxz(a: u32, c: u32) -> u32 {
    cat(pickd(a), pickd(c))
}

fn vxy(a: u32, b: u32) -> u32 {
    cat(pickd(a), pickd(b))
}

static mut RNG_STATE: u32 = 2463534242;

fn rng_next() -> u32 {
    unsafe {
        let mut x = RNG_STATE;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        RNG_STATE = x;
        x
    }
}

fn pick_1_9() -> u32 {
    (rng_next() % 9) + 1
}

fn pick_0_9() -> u32 {
    rng_next() % 10
}

fn main() {
    let val_xyz: [[[u32; 10]; 10]; 9] = [
        [
            [vxyz(1, 0, 0), vxyz(1, 0, 1), vxyz(1, 0, 2), vxyz(1, 0, 3), vxyz(1, 0, 4), vxyz(1, 0, 5), vxyz(1, 0, 6), vxyz(1, 0, 7), vxyz(1, 0, 8), vxyz(1, 0, 9)],
            [vxyz(1, 1, 0), vxyz(1, 1, 1), vxyz(1, 1, 2), vxyz(1, 1, 3), vxyz(1, 1, 4), vxyz(1, 1, 5), vxyz(1, 1, 6), vxyz(1, 1, 7), vxyz(1, 1, 8), vxyz(1, 1, 9)],
            [vxyz(1, 2, 0), vxyz(1, 2, 1), vxyz(1, 2, 2), vxyz(1, 2, 3), vxyz(1, 2, 4), vxyz(1, 2, 5), vxyz(1, 2, 6), vxyz(1, 2, 7), vxyz(1, 2, 8), vxyz(1, 2, 9)],
            [vxyz(1, 3, 0), vxyz(1, 3, 1), vxyz(1, 3, 2), vxyz(1, 3, 3), vxyz(1, 3, 4), vxyz(1, 3, 5), vxyz(1, 3, 6), vxyz(1, 3, 7), vxyz(1, 3, 8), vxyz(1, 3, 9)],
            [vxyz(1, 4, 0), vxyz(1, 4, 1), vxyz(1, 4, 2), vxyz(1, 4, 3), vxyz(1, 4, 4), vxyz(1, 4, 5), vxyz(1, 4, 6), vxyz(1, 4, 7), vxyz(1, 4, 8), vxyz(1, 4, 9)],
            [vxyz(1, 5, 0), vxyz(1, 5, 1), vxyz(1, 5, 2), vxyz(1, 5, 3), vxyz(1, 5, 4), vxyz(1, 5, 5), vxyz(1, 5, 6), vxyz(1, 5, 7), vxyz(1, 5, 8), vxyz(1, 5, 9)],
            [vxyz(1, 6, 0), vxyz(1, 6, 1), vxyz(1, 6, 2), vxyz(1, 6, 3), vxyz(1, 6, 4), vxyz(1, 6, 5), vxyz(1, 6, 6), vxyz(1, 6, 7), vxyz(1, 6, 8), vxyz(1, 6, 9)],
            [vxyz(1, 7, 0), vxyz(1, 7, 1), vxyz(1, 7, 2), vxyz(1, 7, 3), vxyz(1, 7, 4), vxyz(1, 7, 5), vxyz(1, 7, 6), vxyz(1, 7, 7), vxyz(1, 7, 8), vxyz(1, 7, 9)],
            [vxyz(1, 8, 0), vxyz(1, 8, 1), vxyz(1, 8, 2), vxyz(1, 8, 3), vxyz(1, 8, 4), vxyz(1, 8, 5), vxyz(1, 8, 6), vxyz(1, 8, 7), vxyz(1, 8, 8), vxyz(1, 8, 9)],
            [vxyz(1, 9, 0), vxyz(1, 9, 1), vxyz(1, 9, 2), vxyz(1, 9, 3), vxyz(1, 9, 4), vxyz(1, 9, 5), vxyz(1, 9, 6), vxyz(1, 9, 7), vxyz(1, 9, 8), vxyz(1, 9, 9)],
        ],
        [
            [vxyz(2, 0, 0), vxyz(2, 0, 1), vxyz(2, 0, 2), vxyz(2, 0, 3), vxyz(2, 0, 4), vxyz(2, 0, 5), vxyz(2, 0, 6), vxyz(2, 0, 7), vxyz(2, 0, 8), vxyz(2, 0, 9)],
            [vxyz(2, 1, 0), vxyz(2, 1, 1), vxyz(2, 1, 2), vxyz(2, 1, 3), vxyz(2, 1, 4), vxyz(2, 1, 5), vxyz(2, 1, 6), vxyz(2, 1, 7), vxyz(2, 1, 8), vxyz(2, 1, 9)],
            [vxyz(2, 2, 0), vxyz(2, 2, 1), vxyz(2, 2, 2), vxyz(2, 2, 3), vxyz(2, 2, 4), vxyz(2, 2, 5), vxyz(2, 2, 6), vxyz(2, 2, 7), vxyz(2, 2, 8), vxyz(2, 2, 9)],
            [vxyz(2, 3, 0), vxyz(2, 3, 1), vxyz(2, 3, 2), vxyz(2, 3, 3), vxyz(2, 3, 4), vxyz(2, 3, 5), vxyz(2, 3, 6), vxyz(2, 3, 7), vxyz(2, 3, 8), vxyz(2, 3, 9)],
            [vxyz(2, 4, 0), vxyz(2, 4, 1), vxyz(2, 4, 2), vxyz(2, 4, 3), vxyz(2, 4, 4), vxyz(2, 4, 5), vxyz(2, 4, 6), vxyz(2, 4, 7), vxyz(2, 4, 8), vxyz(2, 4, 9)],
            [vxyz(2, 5, 0), vxyz(2, 5, 1), vxyz(2, 5, 2), vxyz(2, 5, 3), vxyz(2, 5, 4), vxyz(2, 5, 5), vxyz(2, 5, 6), vxyz(2, 5, 7), vxyz(2, 5, 8), vxyz(2, 5, 9)],
            [vxyz(2, 6, 0), vxyz(2, 6, 1), vxyz(2, 6, 2), vxyz(2, 6, 3), vxyz(2, 6, 4), vxyz(2, 6, 5), vxyz(2, 6, 6), vxyz(2, 6, 7), vxyz(2, 6, 8), vxyz(2, 6, 9)],
            [vxyz(2, 7, 0), vxyz(2, 7, 1), vxyz(2, 7, 2), vxyz(2, 7, 3), vxyz(2, 7, 4), vxyz(2, 7, 5), vxyz(2, 7, 6), vxyz(2, 7, 7), vxyz(2, 7, 8), vxyz(2, 7, 9)],
            [vxyz(2, 8, 0), vxyz(2, 8, 1), vxyz(2, 8, 2), vxyz(2, 8, 3), vxyz(2, 8, 4), vxyz(2, 8, 5), vxyz(2, 8, 6), vxyz(2, 8, 7), vxyz(2, 8, 8), vxyz(2, 8, 9)],
            [vxyz(2, 9, 0), vxyz(2, 9, 1), vxyz(2, 9, 2), vxyz(2, 9, 3), vxyz(2, 9, 4), vxyz(2, 9, 5), vxyz(2, 9, 6), vxyz(2, 9, 7), vxyz(2, 9, 8), vxyz(2, 9, 9)],
        ],
        [
            [vxyz(3, 0, 0), vxyz(3, 0, 1), vxyz(3, 0, 2), vxyz(3, 0, 3), vxyz(3, 0, 4), vxyz(3, 0, 5), vxyz(3, 0, 6), vxyz(3, 0, 7), vxyz(3, 0, 8), vxyz(3, 0, 9)],
            [vxyz(3, 1, 0), vxyz(3, 1, 1), vxyz(3, 1, 2), vxyz(3, 1, 3), vxyz(3, 1, 4), vxyz(3, 1, 5), vxyz(3, 1, 6), vxyz(3, 1, 7), vxyz(3, 1, 8), vxyz(3, 1, 9)],
            [vxyz(3, 2, 0), vxyz(3, 2, 1), vxyz(3, 2, 2), vxyz(3, 2, 3), vxyz(3, 2, 4), vxyz(3, 2, 5), vxyz(3, 2, 6), vxyz(3, 2, 7), vxyz(3, 2, 8), vxyz(3, 2, 9)],
            [vxyz(3, 3, 0), vxyz(3, 3, 1), vxyz(3, 3, 2), vxyz(3, 3, 3), vxyz(3, 3, 4), vxyz(3, 3, 5), vxyz(3, 3, 6), vxyz(3, 3, 7), vxyz(3, 3, 8), vxyz(3, 3, 9)],
            [vxyz(3, 4, 0), vxyz(3, 4, 1), vxyz(3, 4, 2), vxyz(3, 4, 3), vxyz(3, 4, 4), vxyz(3, 4, 5), vxyz(3, 4, 6), vxyz(3, 4, 7), vxyz(3, 4, 8), vxyz(3, 4, 9)],
            [vxyz(3, 5, 0), vxyz(3, 5, 1), vxyz(3, 5, 2), vxyz(3, 5, 3), vxyz(3, 5, 4), vxyz(3, 5, 5), vxyz(3, 5, 6), vxyz(3, 5, 7), vxyz(3, 5, 8), vxyz(3, 5, 9)],
            [vxyz(3, 6, 0), vxyz(3, 6, 1), vxyz(3, 6, 2), vxyz(3, 6, 3), vxyz(3, 6, 4), vxyz(3, 6, 5), vxyz(3, 6, 6), vxyz(3, 6, 7), vxyz(3, 6, 8), vxyz(3, 6, 9)],
            [vxyz(3, 7, 0), vxyz(3, 7, 1), vxyz(3, 7, 2), vxyz(3, 7, 3), vxyz(3, 7, 4), vxyz(3, 7, 5), vxyz(3, 7, 6), vxyz(3, 7, 7), vxyz(3, 7, 8), vxyz(3, 7, 9)],
            [vxyz(3, 8, 0), vxyz(3, 8, 1), vxyz(3, 8, 2), vxyz(3, 8, 3), vxyz(3, 8, 4), vxyz(3, 8, 5), vxyz(3, 8, 6), vxyz(3, 8, 7), vxyz(3, 8, 8), vxyz(3, 8, 9)],
            [vxyz(3, 9, 0), vxyz(3, 9, 1), vxyz(3, 9, 2), vxyz(3, 9, 3), vxyz(3, 9, 4), vxyz(3, 9, 5), vxyz(3, 9, 6), vxyz(3, 9, 7), vxyz(3, 9, 8), vxyz(3, 9, 9)],
        ],
        [
            [vxyz(4, 0, 0), vxyz(4, 0, 1), vxyz(4, 0, 2), vxyz(4, 0, 3), vxyz(4, 0, 4), vxyz(4, 0, 5), vxyz(4, 0, 6), vxyz(4, 0, 7), vxyz(4, 0, 8), vxyz(4, 0, 9)],
            [vxyz(4, 1, 0), vxyz(4, 1, 1), vxyz(4, 1, 2), vxyz(4, 1, 3), vxyz(4, 1, 4), vxyz(4, 1, 5), vxyz(4, 1, 6), vxyz(4, 1, 7), vxyz(4, 1, 8), vxyz(4, 1, 9)],
            [vxyz(4, 2, 0), vxyz(4, 2, 1), vxyz(4, 2, 2), vxyz(4, 2, 3), vxyz(4, 2, 4), vxyz(4, 2, 5), vxyz(4, 2, 6), vxyz(4, 2, 7), vxyz(4, 2, 8), vxyz(4, 2, 9)],
            [vxyz(4, 3, 0), vxyz(4, 3, 1), vxyz(4, 3, 2), vxyz(4, 3, 3), vxyz(4, 3, 4), vxyz(4, 3, 5), vxyz(4, 3, 6), vxyz(4, 3, 7), vxyz(4, 3, 8), vxyz(4, 3, 9)],
            [vxyz(4, 4, 0), vxyz(4, 4, 1), vxyz(4, 4, 2), vxyz(4, 4, 3), vxyz(4, 4, 4), vxyz(4, 4, 5), vxyz(4, 4, 6), vxyz(4, 4, 7), vxyz(4, 4, 8), vxyz(4, 4, 9)],
            [vxyz(4, 5, 0), vxyz(4, 5, 1), vxyz(4, 5, 2), vxyz(4, 5, 3), vxyz(4, 5, 4), vxyz(4, 5, 5), vxyz(4, 5, 6), vxyz(4, 5, 7), vxyz(4, 5, 8), vxyz(4, 5, 9)],
            [vxyz(4, 6, 0), vxyz(4, 6, 1), vxyz(4, 6, 2), vxyz(4, 6, 3), vxyz(4, 6, 4), vxyz(4, 6, 5), vxyz(4, 6, 6), vxyz(4, 6, 7), vxyz(4, 6, 8), vxyz(4, 6, 9)],
            [vxyz(4, 7, 0), vxyz(4, 7, 1), vxyz(4, 7, 2), vxyz(4, 7, 3), vxyz(4, 7, 4), vxyz(4, 7, 5), vxyz(4, 7, 6), vxyz(4, 7, 7), vxyz(4, 7, 8), vxyz(4, 7, 9)],
            [vxyz(4, 8, 0), vxyz(4, 8, 1), vxyz(4, 8, 2), vxyz(4, 8, 3), vxyz(4, 8, 4), vxyz(4, 8, 5), vxyz(4, 8, 6), vxyz(4, 8, 7), vxyz(4, 8, 8), vxyz(4, 8, 9)],
            [vxyz(4, 9, 0), vxyz(4, 9, 1), vxyz(4, 9, 2), vxyz(4, 9, 3), vxyz(4, 9, 4), vxyz(4, 9, 5), vxyz(4, 9, 6), vxyz(4, 9, 7), vxyz(4, 9, 8), vxyz(4, 9, 9)],
        ],
        [
            [vxyz(5, 0, 0), vxyz(5, 0, 1), vxyz(5, 0, 2), vxyz(5, 0, 3), vxyz(5, 0, 4), vxyz(5, 0, 5), vxyz(5, 0, 6), vxyz(5, 0, 7), vxyz(5, 0, 8), vxyz(5, 0, 9)],
            [vxyz(5, 1, 0), vxyz(5, 1, 1), vxyz(5, 1, 2), vxyz(5, 1, 3), vxyz(5, 1, 4), vxyz(5, 1, 5), vxyz(5, 1, 6), vxyz(5, 1, 7), vxyz(5, 1, 8), vxyz(5, 1, 9)],
            [vxyz(5, 2, 0), vxyz(5, 2, 1), vxyz(5, 2, 2), vxyz(5, 2, 3), vxyz(5, 2, 4), vxyz(5, 2, 5), vxyz(5, 2, 6), vxyz(5, 2, 7), vxyz(5, 2, 8), vxyz(5, 2, 9)],
            [vxyz(5, 3, 0), vxyz(5, 3, 1), vxyz(5, 3, 2), vxyz(5, 3, 3), vxyz(5, 3, 4), vxyz(5, 3, 5), vxyz(5, 3, 6), vxyz(5, 3, 7), vxyz(5, 3, 8), vxyz(5, 3, 9)],
            [vxyz(5, 4, 0), vxyz(5, 4, 1), vxyz(5, 4, 2), vxyz(5, 4, 3), vxyz(5, 4, 4), vxyz(5, 4, 5), vxyz(5, 4, 6), vxyz(5, 4, 7), vxyz(5, 4, 8), vxyz(5, 4, 9)],
            [vxyz(5, 5, 0), vxyz(5, 5, 1), vxyz(5, 5, 2), vxyz(5, 5, 3), vxyz(5, 5, 4), vxyz(5, 5, 5), vxyz(5, 5, 6), vxyz(5, 5, 7), vxyz(5, 5, 8), vxyz(5, 5, 9)],
            [vxyz(5, 6, 0), vxyz(5, 6, 1), vxyz(5, 6, 2), vxyz(5, 6, 3), vxyz(5, 6, 4), vxyz(5, 6, 5), vxyz(5, 6, 6), vxyz(5, 6, 7), vxyz(5, 6, 8), vxyz(5, 6, 9)],
            [vxyz(5, 7, 0), vxyz(5, 7, 1), vxyz(5, 7, 2), vxyz(5, 7, 3), vxyz(5, 7, 4), vxyz(5, 7, 5), vxyz(5, 7, 6), vxyz(5, 7, 7), vxyz(5, 7, 8), vxyz(5, 7, 9)],
            [vxyz(5, 8, 0), vxyz(5, 8, 1), vxyz(5, 8, 2), vxyz(5, 8, 3), vxyz(5, 8, 4), vxyz(5, 8, 5), vxyz(5, 8, 6), vxyz(5, 8, 7), vxyz(5, 8, 8), vxyz(5, 8, 9)],
            [vxyz(5, 9, 0), vxyz(5, 9, 1), vxyz(5, 9, 2), vxyz(5, 9, 3), vxyz(5, 9, 4), vxyz(5, 9, 5), vxyz(5, 9, 6), vxyz(5, 9, 7), vxyz(5, 9, 8), vxyz(5, 9, 9)],
        ],
        [
            [vxyz(6, 0, 0), vxyz(6, 0, 1), vxyz(6, 0, 2), vxyz(6, 0, 3), vxyz(6, 0, 4), vxyz(6, 0, 5), vxyz(6, 0, 6), vxyz(6, 0, 7), vxyz(6, 0, 8), vxyz(6, 0, 9)],
            [vxyz(6, 1, 0), vxyz(6, 1, 1), vxyz(6, 1, 2), vxyz(6, 1, 3), vxyz(6, 1, 4), vxyz(6, 1, 5), vxyz(6, 1, 6), vxyz(6, 1, 7), vxyz(6, 1, 8), vxyz(6, 1, 9)],
            [vxyz(6, 2, 0), vxyz(6, 2, 1), vxyz(6, 2, 2), vxyz(6, 2, 3), vxyz(6, 2, 4), vxyz(6, 2, 5), vxyz(6, 2, 6), vxyz(6, 2, 7), vxyz(6, 2, 8), vxyz(6, 2, 9)],
            [vxyz(6, 3, 0), vxyz(6, 3, 1), vxyz(6, 3, 2), vxyz(6, 3, 3), vxyz(6, 3, 4), vxyz(6, 3, 5), vxyz(6, 3, 6), vxyz(6, 3, 7), vxyz(6, 3, 8), vxyz(6, 3, 9)],
            [vxyz(6, 4, 0), vxyz(6, 4, 1), vxyz(6, 4, 2), vxyz(6, 4, 3), vxyz(6, 4, 4), vxyz(6, 4, 5), vxyz(6, 4, 6), vxyz(6, 4, 7), vxyz(6, 4, 8), vxyz(6, 4, 9)],
            [vxyz(6, 5, 0), vxyz(6, 5, 1), vxyz(6, 5, 2), vxyz(6, 5, 3), vxyz(6, 5, 4), vxyz(6, 5, 5), vxyz(6, 5, 6), vxyz(6, 5, 7), vxyz(6, 5, 8), vxyz(6, 5, 9)],
            [vxyz(6, 6, 0), vxyz(6, 6, 1), vxyz(6, 6, 2), vxyz(6, 6, 3), vxyz(6, 6, 4), vxyz(6, 6, 5), vxyz(6, 6, 6), vxyz(6, 6, 7), vxyz(6, 6, 8), vxyz(6, 6, 9)],
            [vxyz(6, 7, 0), vxyz(6, 7, 1), vxyz(6, 7, 2), vxyz(6, 7, 3), vxyz(6, 7, 4), vxyz(6, 7, 5), vxyz(6, 7, 6), vxyz(6, 7, 7), vxyz(6, 7, 8), vxyz(6, 7, 9)],
            [vxyz(6, 8, 0), vxyz(6, 8, 1), vxyz(6, 8, 2), vxyz(6, 8, 3), vxyz(6, 8, 4), vxyz(6, 8, 5), vxyz(6, 8, 6), vxyz(6, 8, 7), vxyz(6, 8, 8), vxyz(6, 8, 9)],
            [vxyz(6, 9, 0), vxyz(6, 9, 1), vxyz(6, 9, 2), vxyz(6, 9, 3), vxyz(6, 9, 4), vxyz(6, 9, 5), vxyz(6, 9, 6), vxyz(6, 9, 7), vxyz(6, 9, 8), vxyz(6, 9, 9)],
        ],
        [
            [vxyz(7, 0, 0), vxyz(7, 0, 1), vxyz(7, 0, 2), vxyz(7, 0, 3), vxyz(7, 0, 4), vxyz(7, 0, 5), vxyz(7, 0, 6), vxyz(7, 0, 7), vxyz(7, 0, 8), vxyz(7, 0, 9)],
            [vxyz(7, 1, 0), vxyz(7, 1, 1), vxyz(7, 1, 2), vxyz(7, 1, 3), vxyz(7, 1, 4), vxyz(7, 1, 5), vxyz(7, 1, 6), vxyz(7, 1, 7), vxyz(7, 1, 8), vxyz(7, 1, 9)],
            [vxyz(7, 2, 0), vxyz(7, 2, 1), vxyz(7, 2, 2), vxyz(7, 2, 3), vxyz(7, 2, 4), vxyz(7, 2, 5), vxyz(7, 2, 6), vxyz(7, 2, 7), vxyz(7, 2, 8), vxyz(7, 2, 9)],
            [vxyz(7, 3, 0), vxyz(7, 3, 1), vxyz(7, 3, 2), vxyz(7, 3, 3), vxyz(7, 3, 4), vxyz(7, 3, 5), vxyz(7, 3, 6), vxyz(7, 3, 7), vxyz(7, 3, 8), vxyz(7, 3, 9)],
            [vxyz(7, 4, 0), vxyz(7, 4, 1), vxyz(7, 4, 2), vxyz(7, 4, 3), vxyz(7, 4, 4), vxyz(7, 4, 5), vxyz(7, 4, 6), vxyz(7, 4, 7), vxyz(7, 4, 8), vxyz(7, 4, 9)],
            [vxyz(7, 5, 0), vxyz(7, 5, 1), vxyz(7, 5, 2), vxyz(7, 5, 3), vxyz(7, 5, 4), vxyz(7, 5, 5), vxyz(7, 5, 6), vxyz(7, 5, 7), vxyz(7, 5, 8), vxyz(7, 5, 9)],
            [vxyz(7, 6, 0), vxyz(7, 6, 1), vxyz(7, 6, 2), vxyz(7, 6, 3), vxyz(7, 6, 4), vxyz(7, 6, 5), vxyz(7, 6, 6), vxyz(7, 6, 7), vxyz(7, 6, 8), vxyz(7, 6, 9)],
            [vxyz(7, 7, 0), vxyz(7, 7, 1), vxyz(7, 7, 2), vxyz(7, 7, 3), vxyz(7, 7, 4), vxyz(7, 7, 5), vxyz(7, 7, 6), vxyz(7, 7, 7), vxyz(7, 7, 8), vxyz(7, 7, 9)],
            [vxyz(7, 8, 0), vxyz(7, 8, 1), vxyz(7, 8, 2), vxyz(7, 8, 3), vxyz(7, 8, 4), vxyz(7, 8, 5), vxyz(7, 8, 6), vxyz(7, 8, 7), vxyz(7, 8, 8), vxyz(7, 8, 9)],
            [vxyz(7, 9, 0), vxyz(7, 9, 1), vxyz(7, 9, 2), vxyz(7, 9, 3), vxyz(7, 9, 4), vxyz(7, 9, 5), vxyz(7, 9, 6), vxyz(7, 9, 7), vxyz(7, 9, 8), vxyz(7, 9, 9)],
        ],
        [
            [vxyz(8, 0, 0), vxyz(8, 0, 1), vxyz(8, 0, 2), vxyz(8, 0, 3), vxyz(8, 0, 4), vxyz(8, 0, 5), vxyz(8, 0, 6), vxyz(8, 0, 7), vxyz(8, 0, 8), vxyz(8, 0, 9)],
            [vxyz(8, 1, 0), vxyz(8, 1, 1), vxyz(8, 1, 2), vxyz(8, 1, 3), vxyz(8, 1, 4), vxyz(8, 1, 5), vxyz(8, 1, 6), vxyz(8, 1, 7), vxyz(8, 1, 8), vxyz(8, 1, 9)],
            [vxyz(8, 2, 0), vxyz(8, 2, 1), vxyz(8, 2, 2), vxyz(8, 2, 3), vxyz(8, 2, 4), vxyz(8, 2, 5), vxyz(8, 2, 6), vxyz(8, 2, 7), vxyz(8, 2, 8), vxyz(8, 2, 9)],
            [vxyz(8, 3, 0), vxyz(8, 3, 1), vxyz(8, 3, 2), vxyz(8, 3, 3), vxyz(8, 3, 4), vxyz(8, 3, 5), vxyz(8, 3, 6), vxyz(8, 3, 7), vxyz(8, 3, 8), vxyz(8, 3, 9)],
            [vxyz(8, 4, 0), vxyz(8, 4, 1), vxyz(8, 4, 2), vxyz(8, 4, 3), vxyz(8, 4, 4), vxyz(8, 4, 5), vxyz(8, 4, 6), vxyz(8, 4, 7), vxyz(8, 4, 8), vxyz(8, 4, 9)],
            [vxyz(8, 5, 0), vxyz(8, 5, 1), vxyz(8, 5, 2), vxyz(8, 5, 3), vxyz(8, 5, 4), vxyz(8, 5, 5), vxyz(8, 5, 6), vxyz(8, 5, 7), vxyz(8, 5, 8), vxyz(8, 5, 9)],
            [vxyz(8, 6, 0), vxyz(8, 6, 1), vxyz(8, 6, 2), vxyz(8, 6, 3), vxyz(8, 6, 4), vxyz(8, 6, 5), vxyz(8, 6, 6), vxyz(8, 6, 7), vxyz(8, 6, 8), vxyz(8, 6, 9)],
            [vxyz(8, 7, 0), vxyz(8, 7, 1), vxyz(8, 7, 2), vxyz(8, 7, 3), vxyz(8, 7, 4), vxyz(8, 7, 5), vxyz(8, 7, 6), vxyz(8, 7, 7), vxyz(8, 7, 8), vxyz(8, 7, 9)],
            [vxyz(8, 8, 0), vxyz(8, 8, 1), vxyz(8, 8, 2), vxyz(8, 8, 3), vxyz(8, 8, 4), vxyz(8, 8, 5), vxyz(8, 8, 6), vxyz(8, 8, 7), vxyz(8, 8, 8), vxyz(8, 8, 9)],
            [vxyz(8, 9, 0), vxyz(8, 9, 1), vxyz(8, 9, 2), vxyz(8, 9, 3), vxyz(8, 9, 4), vxyz(8, 9, 5), vxyz(8, 9, 6), vxyz(8, 9, 7), vxyz(8, 9, 8), vxyz(8, 9, 9)],
        ],
        [
            [vxyz(9, 0, 0), vxyz(9, 0, 1), vxyz(9, 0, 2), vxyz(9, 0, 3), vxyz(9, 0, 4), vxyz(9, 0, 5), vxyz(9, 0, 6), vxyz(9, 0, 7), vxyz(9, 0, 8), vxyz(9, 0, 9)],
            [vxyz(9, 1, 0), vxyz(9, 1, 1), vxyz(9, 1, 2), vxyz(9, 1, 3), vxyz(9, 1, 4), vxyz(9, 1, 5), vxyz(9, 1, 6), vxyz(9, 1, 7), vxyz(9, 1, 8), vxyz(9, 1, 9)],
            [vxyz(9, 2, 0), vxyz(9, 2, 1), vxyz(9, 2, 2), vxyz(9, 2, 3), vxyz(9, 2, 4), vxyz(9, 2, 5), vxyz(9, 2, 6), vxyz(9, 2, 7), vxyz(9, 2, 8), vxyz(9, 2, 9)],
            [vxyz(9, 3, 0), vxyz(9, 3, 1), vxyz(9, 3, 2), vxyz(9, 3, 3), vxyz(9, 3, 4), vxyz(9, 3, 5), vxyz(9, 3, 6), vxyz(9, 3, 7), vxyz(9, 3, 8), vxyz(9, 3, 9)],
            [vxyz(9, 4, 0), vxyz(9, 4, 1), vxyz(9, 4, 2), vxyz(9, 4, 3), vxyz(9, 4, 4), vxyz(9, 4, 5), vxyz(9, 4, 6), vxyz(9, 4, 7), vxyz(9, 4, 8), vxyz(9, 4, 9)],
            [vxyz(9, 5, 0), vxyz(9, 5, 1), vxyz(9, 5, 2), vxyz(9, 5, 3), vxyz(9, 5, 4), vxyz(9, 5, 5), vxyz(9, 5, 6), vxyz(9, 5, 7), vxyz(9, 5, 8), vxyz(9, 5, 9)],
            [vxyz(9, 6, 0), vxyz(9, 6, 1), vxyz(9, 6, 2), vxyz(9, 6, 3), vxyz(9, 6, 4), vxyz(9, 6, 5), vxyz(9, 6, 6), vxyz(9, 6, 7), vxyz(9, 6, 8), vxyz(9, 6, 9)],
            [vxyz(9, 7, 0), vxyz(9, 7, 1), vxyz(9, 7, 2), vxyz(9, 7, 3), vxyz(9, 7, 4), vxyz(9, 7, 5), vxyz(9, 7, 6), vxyz(9, 7, 7), vxyz(9, 7, 8), vxyz(9, 7, 9)],
            [vxyz(9, 8, 0), vxyz(9, 8, 1), vxyz(9, 8, 2), vxyz(9, 8, 3), vxyz(9, 8, 4), vxyz(9, 8, 5), vxyz(9, 8, 6), vxyz(9, 8, 7), vxyz(9, 8, 8), vxyz(9, 8, 9)],
            [vxyz(9, 9, 0), vxyz(9, 9, 1), vxyz(9, 9, 2), vxyz(9, 9, 3), vxyz(9, 9, 4), vxyz(9, 9, 5), vxyz(9, 9, 6), vxyz(9, 9, 7), vxyz(9, 9, 8), vxyz(9, 9, 9)],
        ],
    ];

    let val_yz: [[u32; 10]; 9] = [
        [vyz(1, 0), vyz(1, 1), vyz(1, 2), vyz(1, 3), vyz(1, 4), vyz(1, 5), vyz(1, 6), vyz(1, 7), vyz(1, 8), vyz(1, 9)],
        [vyz(2, 0), vyz(2, 1), vyz(2, 2), vyz(2, 3), vyz(2, 4), vyz(2, 5), vyz(2, 6), vyz(2, 7), vyz(2, 8), vyz(2, 9)],
        [vyz(3, 0), vyz(3, 1), vyz(3, 2), vyz(3, 3), vyz(3, 4), vyz(3, 5), vyz(3, 6), vyz(3, 7), vyz(3, 8), vyz(3, 9)],
        [vyz(4, 0), vyz(4, 1), vyz(4, 2), vyz(4, 3), vyz(4, 4), vyz(4, 5), vyz(4, 6), vyz(4, 7), vyz(4, 8), vyz(4, 9)],
        [vyz(5, 0), vyz(5, 1), vyz(5, 2), vyz(5, 3), vyz(5, 4), vyz(5, 5), vyz(5, 6), vyz(5, 7), vyz(5, 8), vyz(5, 9)],
        [vyz(6, 0), vyz(6, 1), vyz(6, 2), vyz(6, 3), vyz(6, 4), vyz(6, 5), vyz(6, 6), vyz(6, 7), vyz(6, 8), vyz(6, 9)],
        [vyz(7, 0), vyz(7, 1), vyz(7, 2), vyz(7, 3), vyz(7, 4), vyz(7, 5), vyz(7, 6), vyz(7, 7), vyz(7, 8), vyz(7, 9)],
        [vyz(8, 0), vyz(8, 1), vyz(8, 2), vyz(8, 3), vyz(8, 4), vyz(8, 5), vyz(8, 6), vyz(8, 7), vyz(8, 8), vyz(8, 9)],
        [vyz(9, 0), vyz(9, 1), vyz(9, 2), vyz(9, 3), vyz(9, 4), vyz(9, 5), vyz(9, 6), vyz(9, 7), vyz(9, 8), vyz(9, 9)],
    ];

    let val_xz: [[u32; 10]; 9] = [
        [vxz(1, 0), vxz(1, 1), vxz(1, 2), vxz(1, 3), vxz(1, 4), vxz(1, 5), vxz(1, 6), vxz(1, 7), vxz(1, 8), vxz(1, 9)],
        [vxz(2, 0), vxz(2, 1), vxz(2, 2), vxz(2, 3), vxz(2, 4), vxz(2, 5), vxz(2, 6), vxz(2, 7), vxz(2, 8), vxz(2, 9)],
        [vxz(3, 0), vxz(3, 1), vxz(3, 2), vxz(3, 3), vxz(3, 4), vxz(3, 5), vxz(3, 6), vxz(3, 7), vxz(3, 8), vxz(3, 9)],
        [vxz(4, 0), vxz(4, 1), vxz(4, 2), vxz(4, 3), vxz(4, 4), vxz(4, 5), vxz(4, 6), vxz(4, 7), vxz(4, 8), vxz(4, 9)],
        [vxz(5, 0), vxz(5, 1), vxz(5, 2), vxz(5, 3), vxz(5, 4), vxz(5, 5), vxz(5, 6), vxz(5, 7), vxz(5, 8), vxz(5, 9)],
        [vxz(6, 0), vxz(6, 1), vxz(6, 2), vxz(6, 3), vxz(6, 4), vxz(6, 5), vxz(6, 6), vxz(6, 7), vxz(6, 8), vxz(6, 9)],
        [vxz(7, 0), vxz(7, 1), vxz(7, 2), vxz(7, 3), vxz(7, 4), vxz(7, 5), vxz(7, 6), vxz(7, 7), vxz(7, 8), vxz(7, 9)],
        [vxz(8, 0), vxz(8, 1), vxz(8, 2), vxz(8, 3), vxz(8, 4), vxz(8, 5), vxz(8, 6), vxz(8, 7), vxz(8, 8), vxz(8, 9)],
        [vxz(9, 0), vxz(9, 1), vxz(9, 2), vxz(9, 3), vxz(9, 4), vxz(9, 5), vxz(9, 6), vxz(9, 7), vxz(9, 8), vxz(9, 9)],
    ];

    let val_xy: [[u32; 10]; 9] = [
        [vxy(1, 0), vxy(1, 1), vxy(1, 2), vxy(1, 3), vxy(1, 4), vxy(1, 5), vxy(1, 6), vxy(1, 7), vxy(1, 8), vxy(1, 9)],
        [vxy(2, 0), vxy(2, 1), vxy(2, 2), vxy(2, 3), vxy(2, 4), vxy(2, 5), vxy(2, 6), vxy(2, 7), vxy(2, 8), vxy(2, 9)],
        [vxy(3, 0), vxy(3, 1), vxy(3, 2), vxy(3, 3), vxy(3, 4), vxy(3, 5), vxy(3, 6), vxy(3, 7), vxy(3, 8), vxy(3, 9)],
        [vxy(4, 0), vxy(4, 1), vxy(4, 2), vxy(4, 3), vxy(4, 4), vxy(4, 5), vxy(4, 6), vxy(4, 7), vxy(4, 8), vxy(4, 9)],
        [vxy(5, 0), vxy(5, 1), vxy(5, 2), vxy(5, 3), vxy(5, 4), vxy(5, 5), vxy(5, 6), vxy(5, 7), vxy(5, 8), vxy(5, 9)],
        [vxy(6, 0), vxy(6, 1), vxy(6, 2), vxy(6, 3), vxy(6, 4), vxy(6, 5), vxy(6, 6), vxy(6, 7), vxy(6, 8), vxy(6, 9)],
        [vxy(7, 0), vxy(7, 1), vxy(7, 2), vxy(7, 3), vxy(7, 4), vxy(7, 5), vxy(7, 6), vxy(7, 7), vxy(7, 8), vxy(7, 9)],
        [vxy(8, 0), vxy(8, 1), vxy(8, 2), vxy(8, 3), vxy(8, 4), vxy(8, 5), vxy(8, 6), vxy(8, 7), vxy(8, 8), vxy(8, 9)],
        [vxy(9, 0), vxy(9, 1), vxy(9, 2), vxy(9, 3), vxy(9, 4), vxy(9, 5), vxy(9, 6), vxy(9, 7), vxy(9, 8), vxy(9, 9)],
    ];

    for _ in 0..50 {
        let a = pick_1_9();
        let b = pick_0_9();
        let c = pick_0_9();

        let v1 = val_xyz[(a - 1) as usize][b as usize][c as usize];
        if v1 != 100 * a + 10 * b + c {
            std::process::exit(1);
        }

        let b2 = pick_1_9();
        let c2 = pick_0_9();
        let v2 = val_yz[(b2 - 1) as usize][c2 as usize];
        if v2 != 10 * b2 + c2 {
            std::process::exit(2);
        }

        let a3 = pick_1_9();
        let c3 = pick_0_9();
        let v3 = val_xz[(a3 - 1) as usize][c3 as usize];
        if v3 != 10 * a3 + c3 {
            std::process::exit(3);
        }

        let a4 = pick_1_9();
        let b4 = pick_0_9();
        let v4 = val_xy[(a4 - 1) as usize][b4 as usize];
        if v4 != 10 * a4 + b4 {
            std::process::exit(4);
        }
    }
}