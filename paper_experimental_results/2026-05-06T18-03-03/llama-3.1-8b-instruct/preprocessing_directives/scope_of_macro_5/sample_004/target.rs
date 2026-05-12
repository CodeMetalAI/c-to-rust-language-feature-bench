use rand::Rng;
use rand::SeedableRng;

const RNG_STATE: u32 = 2463534242;

fn rng_next(rng: &mut impl Rng) -> u32 {
    let mut x = rng.gen::<u32>();
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    rng.set_seed(x);
    x
}

fn pick_1_9(rng: &mut impl Rng) -> u32 {
    rng_next(rng) % 9 + 1
}

fn pick_0_9(rng: &mut impl Rng) -> u32 {
    rng_next(rng) % 10
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
    let mut rng = rand::rngs::StdRng::seed_from_u64(RNG_STATE as u64);
    for _ in 0..50 {
        let a = pick_1_9(&mut rng);
        let b = pick_0_9(&mut rng);
        let c = pick_0_9(&mut rng);

        let v1 = val_xyz(a - 1, b, c);
        if v1 != val_xyz(a, b, c) {
            return 1;
        }

        let b2 = pick_1_9(&mut rng);
        let c2 = pick_0_9(&mut rng);
        let v2 = val_yz(b2 - 1, c2);
        if v2 != val_yz(b2, c2) {
            return 2;
        }

        let a3 = pick_1_9(&mut rng);
        let c3 = pick_0_9(&mut rng);
        let v3 = val_xz(a3 - 1, c3);
        if v3 != val_xz(a3, c3) {
            return 3;
        }

        let a4 = pick_1_9(&mut rng);
        let b4 = pick_0_9(&mut rng);
        let v4 = val_xy(a4 - 1, b4);
        if v4 != val_xy(a4, b4) {
            return 4;
        }
    }

    std::process::exit(0);
}