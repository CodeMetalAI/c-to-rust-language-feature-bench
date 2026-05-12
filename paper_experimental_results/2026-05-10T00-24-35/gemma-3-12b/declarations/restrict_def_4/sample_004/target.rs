fn main() {
    let mut x = [0; 32];
    let mut y = [0; 32];
    let mut y_snapshot = [0; 32];

    for i in 0..32 {
        x[i] = 10 + i * 5 + 1;
        y[i] = 100 + i * 5 + 1;
    }

    for i in 0..32 {
        y_snapshot[i] = y[i];
    }

    let gate4 = true;

    if gate4 {
        let mut p1 = &mut x[0..32];
        let mut q1 = &mut y[0..32];

        let mut p2 = &mut x[0..32];
        let mut q2 = &mut y[0..32];
        unsafe {
            std::ptr::copy(
                q2.as_ptr().add(4),
                p2.as_mut().add(8),
                16,
            );
        }

        let expect = [0; 16];
        let mut expect_filled = [0; 16];

        for i in 0..16 {
            expect_filled[i] = y_snapshot[4 + i];
        }

        if !(&x[8..24]).iter().zip(&expect_filled).all(|(a, b)| a == b) {
            std::process::exit(1);
        }

        if !(&y).iter().zip(&y_snapshot).all(|(a, b)| a == b) {
            std::process::exit(2);
        }
    }

    unsafe {
        std::ptr::copy(
            p1.as_ptr().add(0),
            q1.as_mut().add(24),
            8,
        );
    }

    let expect2 = [0; 8];
    let mut expect2_filled = [0; 8];
    for i in 0..8 {
        expect2_filled[i] = y_snapshot[24 + i];
    }

    if !(&x[0..8]).iter().zip(&expect2_filled).all(|(a, b)| a == b) {
        std::process::exit(3);
    }

    if !(&y).iter().zip(&y_snapshot).all(|(a, b)| a == b) {
        std::process::exit(4);
    }

    let sum_x: i32 = x.iter().sum();
    if sum_x == 0 {
        std::process::exit(5);
    }

    std::process::exit(0);
}