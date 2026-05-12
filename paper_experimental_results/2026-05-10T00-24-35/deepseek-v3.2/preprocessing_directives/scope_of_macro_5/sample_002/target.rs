fn main() {
    let mut rng_state: u32 = 2463534242u32;

    fn rng_next(state: &mut u32) -> u32 {
        let mut x = *state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        *state = x;
        x
    }

    fn pick_1_9(state: &mut u32) -> i32 {
        (rng_next(state) % 9u32) as i32 + 1
    }

    fn pick_0_9(state: &mut u32) -> i32 {
        (rng_next(state) % 10u32) as i32
    }

    fn d0() -> i32 { 0 }
    fn d1() -> i32 { 1 }
    fn d2() -> i32 { 2 }
    fn d3() -> i32 { 3 }
    fn d4() -> i32 { 4 }
    fn d5() -> i32 { 5 }
    fn d6() -> i32 { 6 }
    fn d7() -> i32 { 7 }
    fn d8() -> i32 { 8 }
    fn d9() -> i32 { 9 }

    fn pickd(n: i32) -> i32 {
        match n {
            0 => d0(),
            1 => d1(),
            2 => d2(),
            3 => d3(),
            4 => d4(),
            5 => d5(),
            6 => d6(),
            7 => d7(),
            8 => d8(),
            9 => d9(),
            _ => unreachable!(),
        }
    }

    fn vxyz(a: i32, b: i32, c: i32) -> i32 {
        pickd(a) * 100 + pickd(b) * 10 + pickd(c)
    }

    fn vyz(b: i32, c: i32) -> i32 {
        pickd(b) * 10 + pickd(c)
    }

    fn vxz(a: i32, c: i32) -> i32 {
        pickd(a) * 10 + pickd(c)
    }

    fn vxy(a: i32, b: i32) -> i32 {
        pickd(a) * 10 + pickd(b)
    }

    let val_xyz: [[[i32; 10]; 10]; 9] = {
        let mut arr = [[[0; 10]; 10]; 9];
        for a in 1..=9 {
            for b in 0..=9 {
                for c in 0..=9 {
                    arr[a - 1][b][c] = vxyz(a, b, c);
                }
            }
        }
        arr
    };

    let val_yz: [[i32; 10]; 9] = {
        let mut arr = [[0; 10]; 9];
        for b in 1..=9 {
            for c in 0..=9 {
                arr[b - 1][c] = vyz(b, c);
            }
        }
        arr
    };

    let val_xz: [[i32; 10]; 9] = {
        let mut arr = [[0; 10]; 9];
        for a in 1..=9 {
            for c in 0..=9 {
                arr[a - 1][c] = vxz(a, c);
            }
        }
        arr
    };

    let val_xy: [[i32; 10]; 9] = {
        let mut arr = [[0; 10]; 9];
        for a in 1..=9 {
            for b in 0..=9 {
                arr[a - 1][b] = vxy(a, b);
            }
        }
        arr
    };

    for _it in 0..50 {
        let a = pick_1_9(&mut rng_state);
        let b = pick_0_9(&mut rng_state);
        let c = pick_0_9(&mut rng_state);

        let v1 = val_xyz[(a - 1) as usize][b as usize][c as usize];
        if v1 != 100 * a + 10 * b + c {
            return std::process::exit(1);
        }

        let b2 = pick_1_09(&mut rng_state);
        let c2 = pick_0_9(&mut rng_state);
        let v2 = val_yz[(b2 - 1) as usize][c2 as usize];
        if v2 != 10 * b2 + c2 {
            return std::process::exit(2);
        }

        let a3 = pick_1_9(&mut rng_state);
        let c3 = pick_0_9(&mut rng_state);
        let v3 = val_xz[(a3 - 1) as usize][c3 as usize];
        if v3 != 10 * a3 + c3 {
            return std::process::exit(3);
        }

        let a4 = pick_1_9(&mut rng_state);
        let b4 = pick_0_9(&mut rng_state);
        let v4 = val_xy[(a4 - 1) as usize][b4 as usize];
        if v4 != 10 * a4 + b4 {
            return std::process::exit(4);
        }
    }

    std::process::exit(0);
}