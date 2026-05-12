use rand::Rng;
use rand::SeedableRng;

fn rng_next(seed: u32) -> u32 {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
    let x = rng.gen::<u32>();
    let x = x ^ (x << 13);
    let x = x ^ (x >> 17);
    let x = x ^ (x << 5);
    x
}

fn pick_1_9() -> u32 {
    rng_next(2463534242) % 9 + 1
}

fn pick_0_9() -> u32 {
    rng_next(2463534242) % 10
}

fn val_xyz(a: u32, b: u32, c: u32) -> u32 {
    100 * a + 10 * b + c
}

fn val_yz(b: u32, c: u32) -> u32 {
    10 * b + c
}

fn val_xz(a: u32, c: u32) -> u32 {
    10 * a + c
}

fn val_xy(a: u32, b: u32) -> u32 {
    10 * a + b
}

fn main() {
    for _ in 0..50 {
        let a = pick_1_9();
        let b = pick_0_9();
        let c = pick_0_9();

        let v1 = val_xyz(a, b, c);
        if v1!= val_xyz(a, b, c) {
            return 1;
        }

        let b2 = pick_1_9();
        let c2 = pick_0_9();
        let v2 = val_yz(b2, c2);
        if v2!= val_yz(b2, c2) {
            return 2;
        }

        let a3 = pick_1_9();
        let c3 = pick_0_9();
        let v3 = val_xz(a3, c3);
        if v3!= val_xz(a3, c3) {
            return 3;
        }

        let a4 = pick_1_9();
        let b4 = pick_0_9();
        let v4 = val_xy(a4, b4);
        if v4!= val_xy(a4, b4) {
            return 4;
        }
    }

    0
}