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
    let mut x = unsafe { RNG_STATE };
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    unsafe {
        RNG_STATE = x;
    }
    x
}

fn pick_1_9() -> i32 {
    (rng_next() % 9 + 1) as i32
}

fn pick_0_9() -> i32 {
    (rng_next() % 10) as i32
}

fn main() {
    let val_xyz: Vec<Vec<Vec<u32>>> = (1..=9)
        .map(|a| {
            (0..=9)
                .map(|b| (0..=9).map(|c| vxyz(a, b, c)).collect())
                .collect()
        })
        .collect();

    let val_yz: Vec<Vec<u32>> = (1..=9)
        .map(|b| (0..=9).map(|c| vyz(b, c)).collect())
        .collect();

    let val_xz: Vec<Vec<u32>> = (1..=9)
        .map(|a| (0..=9).map(|c| vxz(a, c)).collect())
        .collect();

    let val_xy: Vec<Vec<u32>> = (1..=9)
        .map(|a| (0..=9).map(|b| vxy(a, b)).collect())
        .collect();

    for _ in 0..50 {
        let a = pick_1_9() as usize;
        let b = pick_0_9() as usize;
        let c = pick_0_9() as usize;

        let v1 = val_xyz[a - 1][b][c];
        if v1 != 100 * a as u32 + 10 * b as u32 + c as u32 {
            std::process::exit(1);
        }

        let b2 = pick_1_9() as usize;
        let c2 = pick_0_9() as usize;
        let v2 = val_yz[b2 - 1][c2];
        if v2 != 10 * b2 as u32 + c2 as u32 {
            std::process::exit(2);
        }

        let a3 = pick_1_9() as usize;
        let c3 = pick_0_9() as usize;
        let v3 = val_xz[a3 - 1][c3];
        if v3 != 10 * a3 as u32 + c3 as u32 {
            std::process::exit(3);
        }

        let a4 = pick_1_9() as usize;
        let b4 = pick_0_9() as usize;
        let v4 = val_xy[a4 - 1][b4];
        if v4 != 10 * a4 as u32 + b4 as u32 {
            std::process::exit(4);
        }
    }
}