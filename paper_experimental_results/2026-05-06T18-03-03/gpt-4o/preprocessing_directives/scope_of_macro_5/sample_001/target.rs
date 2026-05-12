fn cat(a: u32, b: u32) -> u32 {
    a * 10 + b
}

fn xcat(a: u32, b: u32) -> u32 {
    cat(a, b)
}

fn cat3(a: u32, b: u32, c: u32) -> u32 {
    xcat(xcat(a, b), c)
}

fn pickd(n: u32) -> u32 {
    n
}

fn vxyz(a: u32, b: u32, c: u32) -> u32 {
    cat3(pickd(a), pickd(b), pickd(c))
}

fn vyz(b: u32, c: u32) -> u32 {
    xcat(pickd(b), pickd(c))
}

fn vxz(a: u32, c: u32) -> u32 {
    xcat(pickd(a), pickd(c))
}

fn vxy(a: u32, b: u32) -> u32 {
    xcat(pickd(a), pickd(b))
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

fn pick_1_9() -> i32 {
    (rng_next() % 9 + 1) as i32
}

fn pick_0_9() -> i32 {
    (rng_next() % 10) as i32
}

fn main() {
    let val_xyz: [[[u32; 10]; 10]; 9] = [
        [vxyz(1, 0, 0), vxyz(1, 0, 1), vxyz(1, 0, 2), vxyz(1, 0, 3), vxyz(1, 0, 4), vxyz(1, 0, 5), vxyz(1, 0, 6), vxyz(1, 0, 7), vxyz(1, 0, 8), vxyz(1, 0, 9)],
        [vxyz(2, 0, 0), vxyz(2, 0, 1), vxyz(2, 0, 2), vxyz(2, 0, 3), vxyz(2, 0, 4), vxyz(2, 0, 5), vxyz(2, 0, 6), vxyz(2, 0, 7), vxyz(2, 0, 8), vxyz(2, 0, 9)],
        [vxyz(3, 0, 0), vxyz(3, 0, 1), vxyz(3, 0, 2), vxyz(3, 0, 3), vxyz(3, 0, 4), vxyz(3, 0, 5), vxyz(3, 0, 6), vxyz(3, 0, 7), vxyz(3, 0, 8), vxyz(3, 0, 9)],
        [vxyz(4, 0, 0), vxyz(4, 0, 1), vxyz(4, 0, 2), vxyz(4, 0, 3), vxyz(4, 0, 4), vxyz(4, 0, 5), vxyz(4, 0, 6), vxyz(4, 0, 7), vxyz(4, 0, 8), vxyz(4, 0, 9)],
        [vxyz(5, 0, 0), vxyz(5, 0, 1), vxyz(5, 0, 2), vxyz(5, 0, 3), vxyz(5, 0, 4), vxyz(5, 0, 5), vxyz(5, 0, 6), vxyz(5, 0, 7), vxyz(5, 0, 8), vxyz(5, 0, 9)],
        [vxyz(6, 0, 0), vxyz(6, 0, 1), vxyz(6, 0, 2), vxyz(6, 0, 3), vxyz(6, 0, 4), vxyz(6, 0, 5), vxyz(6, 0, 6), vxyz(6, 0, 7), vxyz(6, 0, 8), vxyz(6, 0, 9)],
        [vxyz(7, 0, 0), vxyz(7, 0, 1), vxyz(7, 0, 2), vxyz(7, 0, 3), vxyz(7, 0, 4), vxyz(7, 0, 5), vxyz(7, 0, 6), vxyz(7, 0, 7), vxyz(7, 0, 8), vxyz(7, 0, 9)],
        [vxyz(8, 0, 0), vxyz(8, 0, 1), vxyz(8, 0, 2), vxyz(8, 0, 3), vxyz(8, 0, 4), vxyz(8, 0, 5), vxyz(8, 0, 6), vxyz(8, 0, 7), vxyz(8, 0, 8), vxyz(8, 0, 9)],
        [vxyz(9, 0, 0), vxyz(9, 0, 1), vxyz(9, 0, 2), vxyz(9, 0, 3), vxyz(9, 0, 4), vxyz(9, 0, 5), vxyz(9, 0, 6), vxyz(9, 0, 7), vxyz(9, 0, 8), vxyz(9, 0, 9)],
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
        if v1 != 100 * a as u32 + 10 * b as u32 + c as u32 {
            std::process::exit(1);
        }

        let b2 = pick_1_9();
        let c2 = pick_0_9();
        let v2 = val_yz[(b2 - 1) as usize][c2 as usize];
        if v2 != 10 * b2 as u32 + c2 as u32 {
            std::process::exit(2);
        }

        let a3 = pick_1_9();
        let c3 = pick_0_9();
        let v3 = val_xz[(a3 - 1) as usize][c3 as usize];
        if v3 != 10 * a3 as u32 + c3 as u32 {
            std::process::exit(3);
        }

        let a4 = pick_1_9();
        let b4 = pick_0_9();
        let v4 = val_xy[(a4 - 1) as usize][b4 as usize];
        if v4 != 10 * a4 as u32 + b4 as u32 {
            std::process::exit(4);
        }
    }
}