fn main() {
    const D0: usize = 0;
    const D1: usize = 1;
    const D2: usize = 2;
    const D3: usize = 3;
    const D4: usize = 4;
    const D5: usize = 5;
    const D6: usize = 6;
    const D7: usize = 7;
    const D8: usize = 8;
    const D9: usize = 9;

    fn pick_d(n: usize) -> usize {
        match n {
            0 => D0,
            1 => D1,
            2 => D2,
            3 => D3,
            4 => D4,
            5 => D5,
            6 => D6,
            7 => D7,
            8 => D8,
            9 => D9,
            _ => unreachable!(),
        }
    }

    fn v_xyz(a: usize, b: usize, c: usize) -> usize {
        100 * pick_d(a) + 10 * pick_d(b) + pick_d(c)
    }

    fn v_yz(b: usize, c: usize) -> usize {
        10 * pick_d(b) + pick_d(c)
    }

    fn v_xz(a: usize, c: usize) -> usize {
        10 * pick_d(a) + pick_d(c)
    }

    fn v_xy(a: usize, b: usize) -> usize {
        10 * pick_d(a) + pick_d(b)
    }

    let mut rng_state: u32 = 2463534242;

    fn rng_next(state: &mut u32) -> u32 {
        let x = *state;
        let new_x = x ^ (x << 13) ^ (x >> 17) ^ (x << 5);
        *state = new_x;
        new_x
    }

    fn pick_1_9(state: &mut u32) -> usize {
        (rng_next(state) % 9) as usize + 1
    }

    fn pick_0_9(state: &mut u32) -> usize {
        (rng_next(state) % 10) as usize
    }

    let val_xyz: [[[usize; 10]; 9]; 10] = [
        [
            [v_xyz(1, 0, 0), v_xyz(1, 0, 1), v_xyz(1, 0, 2), v_xyz(1, 0, 3), v_xyz(1, 0, 4), v_xyz(1, 0, 5), v_xyz(1, 0, 6), v_xyz(1, 0, 7), v_xyz(1, 0, 8), v_xyz(1, 0, 9)],
            [v_xyz(1, 1, 0), v_xyz(1, 1, 1), v_xyz(1, 1, 2), v_xyz(1, 1, 3), v_xyz(1, 1, 4), v_xyz(1, 1, 5), v_xyz(1, 1, 6), v_xyz(1, 1, 7), v_xyz(1, 1, 8), v_xyz(1, 1, 9)],
            [v_xyz(1, 2, 0), v_xyz(1, 2, 1), v_xyz(1, 2, 2), v_xyz(1, 2, 3), v_xyz(1, 2, 4), v_xyz(1, 2, 5), v_xyz(1, 2, 6), v_xyz(1, 2, 7), v_xyz(1, 2, 8), v_xyz(1, 2, 9)],
            [v_xyz(1, 3, 0), v_xyz(1, 3, 1), v_xyz(1, 3, 2), v_xyz(1, 3, 3), v_xyz(1, 3, 4), v_xyz(1, 3, 5), v_xyz(1, 3, 6), v_xyz(1, 3, 7), v_xyz(1, 3, 8), v_xyz(1, 3, 9)],
            [v_xyz(1, 4, 0), v_xyz(1, 4, 1), v_xyz(1, 4, 2), v_xyz(1, 4, 3), v_xyz(1, 4, 4), v_xyz(1, 4, 5), v_xyz(1, 4, 6), v_xyz(1, 4, 7), v_xyz(1, 4, 8), v_xyz(1, 4, 9)],
            [v_xyz(1, 5, 0), v_xyz(1, 5, 1), v_xyz(1, 5, 2), v_xyz(1, 5, 3), v_xyz(1, 5, 4), v_xyz(1, 5, 5), v_xyz(1, 5, 6), v_xyz(1, 5, 7), v_xyz(1, 5, 8), v_xyz(1, 5, 9)],
            [v_xyz(1, 6, 0), v_xyz(1, 6, 1), v_xyz(1, 6, 2), v_xyz(1, 6, 3), v_xyz(1, 6, 4), v_xyz(1, 6, 5), v_xyz(1, 6, 6), v_xyz(1, 6, 7), v_xyz(1, 6, 8), v_xyz(1, 6, 9)],
            [v_xyz(1, 7, 0), v_xyz(1, 7, 1), v_xyz(1, 7, 2), v_xyz(1, 7, 3), v_xyz(1, 7, 4), v_xyz(1, 7, 5), v_xyz(1, 7, 6), v_xyz(1, 7, 7), v_xyz(1, 7, 8), v_xyz(1, 7, 9)],
            [v_xyz(1, 8, 0), v_xyz(1, 8, 1), v_xyz(1, 8, 2), v_xyz(1, 8, 3), v_xyz(1, 8, 4), v_xyz(1, 8, 5), v_xyz(1, 8, 6), v_xyz(1, 8, 7), v_xyz(1, 8, 8), v_xyz(1, 8, 9)],
            [v_xyz(1, 9, 0), v_xyz(1, 9, 1), v_xyz(1, 9, 2), v_xyz(1, 9, 3), v_xyz(1, 9, 4), v_xyz(1, 9, 5), v_xyz(1, 9, 6), v_xyz(1, 9, 7), v_xyz(1, 9, 8), v_xyz(1, 9, 9)],
        ],
        // Repeat for other rows...
    ];

    let val_yz: [[usize; 10]; 9] = [
        [v_yz(1, 0), v_yz(1, 1), v_yz(1, 2), v_yz(1, 3), v_yz(1, 4), v_yz(1, 5), v_yz(1, 6), v_yz(1, 7), v_yz(1, 8), v_yz(1, 9)],
        // Repeat for other rows...
    ];

    let val_xz: [[usize; 10]; 9] = [
        [v_xz(1, 0), v_xz(1, 1), v_xz(1, 2), v_xz(1, 3), v_xz(1, 4), v_xz(1, 5), v_xz(1, 6), v_xz(1, 7), v_xz(1, 8), v_xz(1, 9)],
        // Repeat for other rows...
    ];

    let val_xy: [[usize; 10]; 9] = [
        [v_xy(1, 0), v_xy(1, 1), v_xy(1, 2), v_xy(1, 3), v_xy(1, 4), v_xy(1, 5), v_xy(1, 6), v_xy(1, 7), v_xy(1, 8), v_xy(1, 9)],
        // Repeat for other rows...
    ];

    for _ in 0..50 {
        let a = pick_1_9(&mut rng_state);
        let b = pick_0_9(&mut rng_state);
        let c = pick_0_9(&mut rng_state);

        let v1 = val_xyz[a - 1][b][c];
        if v1 != 100 * a + 10 * b + c {
            std::process::exit(1);
        }

        let b2 = pick_1_9(&mut rng_state);
        let c2 = pick_0_9(&mut rng_state);
        let v2 = val_yz[b2 - 1][c2];
        if v2 != 10 * b2 + c2 {
            std::process::exit(2);
        }

        let a3 = pick_1_9(&mut rng_state);
        let c3 = pick_0_9(&mut rng_state);
        let v3 = val_xz[a3 - 1][c3];
        if v3 != 10 * a3 + c3 {
            std::process::exit(3);
        }

        let a4 = pick_1_9(&mut rng_state);
        let b4 = pick_0_9(&mut rng_state);
        let v4 = val_xy[a4 - 1][b4];
        if v4 != 10 * a4 + b4 {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}