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
                p2.as_mut_ptr().add(8),
                16,
            );
        }

        let expect = [0; 16];
        let mut i = 0;
        while i < 16 {
            let index = 4 + i;
            expect[i] = y_snapshot[index];
            i += 1;
        }

        if !(&x[8..24]).iter().zip(&expect).all(|(a, b)| a == b) {
            return 1;
        }

        if !(&y[0..32]).iter().zip(&y_snapshot).all(|(a, b)| a == b) {
            return 2;
        }
    }

    unsafe {
        std::ptr::copy(
            p1.as_ptr().add(0),
            q1.as_mut_ptr().add(24),
            8,
        );
    }

    let expect2 = [0; 8];
    let mut i = 0;
    while i < 8 {
        let index = 24 + i;
        expect2[i] = y_snapshot[index];
        i += 1;
    }

    if !(&x[0..8]).iter().zip(&expect2).all(|(a, b)| a == b) {
        return 3;
    }

    if !(&y[0..32]).iter().zip(&y_snapshot).all(|(a, b)| a == b) {
        return 4;
    }

    let sum_x: i32 = x.iter().sum();
    if sum_x == 0 {
        return 5;
    }

    0
}