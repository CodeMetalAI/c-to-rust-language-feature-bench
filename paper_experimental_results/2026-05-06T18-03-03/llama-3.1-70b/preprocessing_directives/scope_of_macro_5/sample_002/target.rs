fn rng_next(state: &mut u32) -> u32 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *state = x;
    x
}

fn pick_1_9(state: &mut u32) -> u32 {
    (rng_next(state) % 9) + 1
}

fn pick_0_9(state: &mut u32) -> u32 {
    rng_next(state) % 10
}

fn main() {
    let mut rng_state = 2463534242u;
    for _ in 0..50 {
        let a = pick_1_9(&mut rng_state) as usize;
        let b = pick_0_9(&mut rng_state) as usize;
        let c = pick_0_9(&mut rng_state) as usize;

        let v1 = 100 * a + 10 * b + c;
        if v1!= 100 * a + 10 * b + c {
            std::process::exit(1);
        }

        let b2 = pick_1_9(&mut rng_state) as usize;
        let c2 = pick_0_9(&mut rng_state) as usize;
        let v2 = 10 * b2 + c2;
        if v2!= 10 * b2 + c2 {
            std::process::exit(2);
        }

        let a3 = pick_1_9(&mut rng_state) as usize;
        let c3 = pick_0_9(&mut rng_state) as usize;
        let v3 = 10 * a3 + c3;
        if v3!= 10 * a3 + c3 {
            std::process::exit(3);
        }

        let a4 = pick_1_9(&mut rng_state) as usize;
        let b4 = pick_0_9(&mut rng_state) as usize;
        let v4 = 10 * a4 + b4;
        if v4!= 10 * a4 + b4 {
            std::process::exit(4);
        }
    }
}