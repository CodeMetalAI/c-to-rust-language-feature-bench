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
            _ => panic!("Invalid number"),
        }
    }

    fn v_xyz(a: usize, b: usize, c: usize) -> usize {
        v_xyz_inner(pick_d(a), pick_d(b), pick_d(c))
    }

    fn v_xyz_inner(a: usize, b: usize, c: usize) -> usize {
        100 * a + 10 * b + c
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

    static mut RNG_STATE: u32 = 2463534242;

    fn rng_next() -> u32 {
        unsafe {
            let x = RNG_STATE;
            let new_x = x ^ (x << 13);
            let new_x = new_x ^ (new_x >> 17);
            let new_x = new_x ^ (new_x << 5);
            RNG_STATE = new_x;
            new_x
        }
    }

    fn pick_1_9() -> usize {
        (rng_next() % 9) as usize + 1
    }

    fn pick_0_9() -> usize {
        (rng_next() % 10) as usize
    }

    let val_xyz: [[usize; 10]; 9] = [
        [v_xyz(1, 0, 0), v_xyz(1, 0, 1), v_xyz(1, 0, 2), v_xyz(1, 0, 3), v_xyz(1, 0, 4), v_xyz(1, 0, 5), v_xyz(1, 0, 6), v_xyz(1, 0, 7), v_xyz(1, 0, 8), v_xyz(1, 0, 9)],
        [v_xyz(2, 0, 0), v_xyz(2, 0, 1), v_xyz(2, 0, 2), v_xyz(2, 0, 3), v_xyz(2, 0, 4), v_xyz(2, 0, 5), v_xyz(2, 0, 6), v_xyz(2, 0, 7), v_xyz(2, 0, 8), v_xyz(2, 0, 9)],
        [v_xyz(3, 0, 0), v_xyz(3, 0, 1), v_xyz(3, 0, 2), v_xyz(3, 0, 3), v_xyz(3, 0, 4), v_xyz(3, 0, 5), v_xyz(3, 0, 6), v_xyz(3, 0, 7), v_xyz(3, 0, 8), v_xyz(3, 0, 9)],
        [v_xyz(4, 0, 0), v_xyz(4, 0, 1), v_xyz(4, 0, 2), v_xyz(4, 0, 3), v_xyz(4, 0, 4), v_xyz(4, 0, 5), v_xyz(4, 0, 6), v_xyz(4, 0, 7), v_xyz(4, 0, 8), v_xyz(4, 0, 9)],
        [v_xyz(5, 0, 0), v_xyz(5, 0, 1), v_xyz(5, 0, 2), v_xyz(5, 0, 3), v_xyz(5, 0, 4), v_xyz(5, 0, 5), v_xyz(5, 0, 6), v_xyz(5, 0, 7), v_xyz(5, 0, 8), v_xyz(5, 0, 9)],
        [v_xyz(6, 0, 0), v_xyz(6, 0, 1), v_xyz(6, 0, 2), v_xyz(6, 0, 3), v_xyz(6, 0, 4), v_xyz(6, 0, 5), v_xyz(6, 0, 6), v_xyz(6, 0, 7), v_xyz(6, 0, 8), v_xyz(6, 0, 9)],
        [v_xyz(7, 0, 0), v_xyz(7, 0, 1), v_xyz(7, 0, 2), v_xyz(7, 0, 3), v_xyz(7, 0, 4), v_xyz(7, 0, 5), v_xyz(7, 0, 6), v_xyz(7, 0, 7), v_xyz(7, 0, 8), v_xyz(7, 0, 9)],
        [v_xyz(8, 0, 0), v_xyz(8, 0, 1), v_xyz(8, 0, 2), v_xyz(8, 0, 3), v_xyz(8, 0, 4), v_xyz(8, 0, 5), v_xyz(8, 0, 6), v_xyz(8, 0, 7), v_xyz(8, 0, 8), v_xyz(8, 0, 9)],
        [v_xyz(9, 0, 0), v_xyz(9, 0, 1), v_xyz(9, 0, 2), v_xyz(9, 0, 3), v_xyz(9, 0, 4), v_xyz(9, 0, 5), v_xyz(9, 0, 6), v_xyz(9, 0, 7), v_xyz(9, 0, 8), v_xyz(9, 0, 9)],
    ];

    let val_yz: [[usize; 10]; 9] = [
        [v_yz(1, 0), v_yz(1, 1), v_yz(1, 2), v_yz(1, 3), v_yz(1, 4), v_yz(1, 5), v_yz(1, 6), v_yz(1, 7), v_yz(1, 8), v_yz(1, 9)],
        [v_yz(2, 0), v_yz(2, 1), v_yz(2, 2), v_yz(2, 3), v_yz(2, 4), v_yz(2, 5), v_yz(2, 6), v_yz(2, 7), v_yz(2, 8), v_yz(2, 9)],
        [v_yz(3, 0), v_yz(3, 1), v_yz(3, 2), v_yz(3, 3), v_yz(3, 4), v_yz(3, 5), v_yz(3, 6), v_yz(3, 7), v_yz(3, 8), v_yz(3, 9)],
        [v_yz(4, 0), v_yz(4, 1), v_yz(4, 2), v_yz(4, 3), v_yz(4, 4), v_yz(4, 5), v_yz(4, 6), v_yz(4, 7), v_yz(4, 8), v_yz(4, 9)],
        [v_yz(5, 0), v_yz(5, 1), v_yz(5, 2), v_yz(5, 3), v_yz(5, 4), v_yz(5, 5), v_yz(5, 6), v_yz(5, 7), v_yz(5, 8), v_yz(5, 9)],
        [v_yz(6, 0), v_yz(6, 1), v_yz(6, 2), v_yz(6, 3), v_yz(6, 4), v_yz(6, 5), v_yz(6, 6), v_yz(6, 7), v_yz(6, 8), v_yz(6, 9)],
        [v_yz(7, 0), v_yz(7, 1), v_yz(7, 2), v_yz(7, 3), v_yz(7, 4), v_yz(7, 5), v_yz(7, 6), v_yz(7, 7), v_yz(7, 8), v_yz(7, 9)],
        [v_yz(8, 0), v_yz(8, 1), v_yz(8, 2), v_yz(8, 3), v_yz(8, 4), v_yz(8, 5), v_yz(8, 6), v_yz(8, 7), v_yz(8, 8), v_yz(8, 9)],
        [v_yz(9, 0), v_yz(9, 1), v_yz(9, 2), v_yz(9, 3), v_yz(9, 4), v_yz(9, 5), v_yz(9, 6), v_yz(9, 7), v_yz(9, 8), v_yz(9, 9)],
    ];

    let val_xz: [[usize; 10]; 9] = [
        [v_xz(1, 0), v_xz(1, 1), v_xz(1, 2), v_xz(1, 3), v_xz(1, 4), v_xz(1, 5), v_xz(1, 6), v_xz(1, 7), v_xz(1, 8), v_xz(1, 9)],
        [v_xz(2, 0), v_xz(2, 1), v_xz(2, 2), v_xz(2, 3), v_xz(2, 4), v_xz(2, 5), v_xz(2, 6), v_xz(2, 7), v_xz(2, 8), v_xz(2, 9)],
        [v_xz(3, 0), v_xz(3, 1), v_xz(3, 2), v_xz(3, 3), v_xz(3, 4), v_xz(3, 5), v_xz(3, 6), v_xz(3, 7), v_xz(3, 8), v_xz(3, 9)],
        [v_xz(4, 0), v_xz(4, 1), v_xz(4, 2), v_xz(4, 3), v_xz(4, 4), v_xz(4, 5), v_xz(4, 6), v_xz(4, 7), v_xz(4, 8), v_xz(4, 9)],
        [v_xz(5, 0), v_xz(5, 1), v_xz(5, 2), v_xz(5, 3), v_xz(5, 4), v_xz(5, 5), v_xz(5, 6), v_xz(5, 7), v_xz(5, 8), v_xz(5, 9)],
        [v_xz(6, 0), v_xz(6, 1), v_xz(6, 2), v_xz(6, 3), v_xz(6, 4), v_xz(6, 5), v_xz(6, 6), v_xz(6, 7), v_xz(6, 8), v_xz(6, 9)],
        [v_xz(7, 0), v_xz(7, 1), v_xz(7, 2), v_xz(7, 3), v_xz(7, 4), v_xz(7, 5), v_xz(7, 6), v_xz(7, 7), v_xz(7, 8), v_xz(7, 9)],
        [v_xz(8, 0), v_xz(8, 1), v_xz(8, 2), v_xz(8, 3), v_xz(8, 4), v_xz(8, 5), v_xz(8, 6), v_xz(8, 7), v_xz(8, 8), v_xz(8, 9)],
        [v_xz(9, 0), v_xz(9, 1), v_xz(9, 2), v_xz(9, 3), v_xz(9, 4), v_xz(9, 5), v_xz(9, 6), v_xz(9, 7), v_xz(9, 8), v_xz(9, 9)],
    ];

    let val_xy: [[usize; 10]; 9] = [
        [v_xy(1, 0), v_xy(1, 1), v_xy(1, 2), v_xy(1, 3), v_xy(1, 4), v_xy(1, 5), v_xy(1, 6), v_xy(1, 7), v_xy(1, 8), v_xy(1, 9)],
        [v_xy(2, 0), v_xy(2, 1), v_xy(2, 2), v_xy(2, 3), v_xy(2, 4), v_xy(2, 5), v_xy(2, 6), v_xy(2, 7), v_xy(2, 8), v_xy(2, 9)],
        [v_xy(3, 0), v_xy(3, 1), v_xy(3, 2), v_xy(3, 3), v_xy(3, 4), v_xy(3, 5), v_xy(3, 6), v_xy(3, 7), v_xy(3, 8), v_xy(3, 9)],
        [v_xy(4, 0), v_xy(4, 1), v_xy(4, 2), v_xy(4, 3), v_xy(4, 4), v_xy(4, 5), v_xy(4, 6), v_xy(4, 7), v_xy(4, 8), v_xy(4, 9)],
        [v_xy(5, 0), v_xy(5, 1), v_xy(5, 2), v_xy(5, 3), v_xy(5, 4), v_xy(5, 5), v_xy(5, 6), v_xy(5, 7), v_xy(5, 8), v_xy(5, 9)],
        [v_xy(6, 0), v_xy(6, 1), v_xy(6, 2), v_xy(6, 3), v_xy(6, 4), v_xy(6, 5), v_xy(6, 6), v_xy(6, 7), v_xy(6, 8), v_xy(6, 9)],
        [v_xy(7, 0), v_xy(7, 1), v_xy(7, 2), v_xy(7, 3), v_xy(7, 4), v_xy(7, 5), v_xy(7, 6), v_xy(7, 7), v_xy(7, 8), v_xy(7, 9)],
        [v_xy(8, 0), v_xy(8, 1), v_xy(8, 2), v_xy(8, 3), v_xy(8, 4), v_xy(8, 5), v_xy(8, 6), v_xy(8, 7), v_xy(8, 8), v_xy(8, 9)],
        [v_xy(9, 0), v_xy(9, 1), v_xy(9, 2), v_xy(9, 3), v_xy(9, 4), v_xy(9, 5), v_xy(9, 6), v_xy(9, 7), v_xy(9, 8), v_xy(9, 9)],
    ];

    for _ in 0..50 {
        let a = pick_1_9();
        let b = pick_0_9();
        let c = pick_0_9();

        let v1 = val_xyz[a - 1][b][c];
        if v1 != 100 * a + 10 * b + c {
            std::process::exit(1);
        }

        let b2 = pick_1_9();
        let c2 = pick_0_9();
        let v2 = val_yz[b2 - 1][c2];
        if v2 != 10 * b2 + c2 {
            std::process::exit(2);
        }

        let a3 = pick_1_9();
        let c3 = pick_0_9();
        let v3 = val_xz[a3 - 1][c3];
        if v3 != 10 * a3 + c3 {
            std::process::exit(3);
        }

        let a4 = pick_1_9();
        let b4 = pick_0_9();
        let v4 = val_xy[a4 - 1][b4];
        if v4 != 10 * a4 + b4 {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}