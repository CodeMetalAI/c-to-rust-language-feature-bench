use rand::Rng;

const RNG_STATE: u32 = 2463534242;

fn rng_next() -> u32 {
    let mut x = RNG_STATE;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    RNG_STATE = x;
    x
}

fn pick_1_9() -> u32 {
    (rng_next() % 9u32) + 1
}

fn pick_0_9() -> u32 {
    rng_next() % 10u32
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
    let mut rng = rand::thread_rng();
    for _ in 0..50 {
        let a = pick_1_9();
        let b = pick_0_9();
        let c = pick_0_9();

        let v1 = val_xyz(a, b, c);
        if v1!= val_xyz(a, b, c) {
            panic!("v1 mismatch");
        }

        let b2 = pick_1_9();
        let c2 = pick_0_9();
        let v2 = val_yz(b2, c2);
        if v2!= val_yz(b2, c2) {
            panic!("v2 mismatch");
        }

        let a3 = pick_1_9();
        let c3 = pick_0_9();
        let v3 = val_xz(a3, c3);
        if v3!= val_xz(a3, c3) {
            panic!("v3 mismatch");
        }

        let a4 = pick_1_9();
        let b4 = pick_0_9();
        let v4 = val_xy(a4, b4);
        if v4!= val_xy(a4, b4) {
            panic!("v4 mismatch");
        }
    }
}