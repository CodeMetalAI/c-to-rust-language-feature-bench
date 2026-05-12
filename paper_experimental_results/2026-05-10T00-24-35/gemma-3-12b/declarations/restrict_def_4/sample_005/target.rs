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
        let mut i = 0;
        while i < 16 {
            expect[i] = y_snapshot[4 + i];
            i += 1;
        }

        if !(&x[8..24]).iter().zip(&expect).all(|(a, b)| a == b) {
            std::process::exit(1);
        }


        if !(&y[0..32]).iter().zip(&y_snapshot).all(|(a, b)| a == b) {
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
    let mut i = 0;
    while i < 8 {
        expect2[i] = y_snapshot[24 + i];
        i += 1;
    }

    if !(&x[0..8]).iter().zip(&expect2).all(|(a, b)| a == b) {
        std::process::exit(3);
    }

    if !(&y[0..32]).iter().zip(&y_snapshot).all(|(a, b)| a == b) {
        std::process::exit(4);
    }

    let sum_x: i32 = x.iter().sum();
    if sum_x == 0 {
        std::process::exit(5);
    }

    std::process::exit(0);
}