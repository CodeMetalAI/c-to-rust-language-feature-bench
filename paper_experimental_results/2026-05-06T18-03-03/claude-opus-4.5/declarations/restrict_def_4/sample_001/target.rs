static mut GATE4: i32 = 1;

fn copy_restrict(mut n: i32, p: &mut [i32], q: &[i32]) {
    let mut pi = 0;
    let mut qi = 0;
    while n > 0 {
        n -= 1;
        p[pi] = q[qi];
        pi += 1;
        qi += 1;
    }
}

fn fill(p: &mut [i32], n: i32, base: i32) {
    let mut i = 0;
    while i < n {
        p[i as usize] = base + i * 5 + 1;
        i += 1;
    }
}

fn same(a: &[i32], b: &[i32], n: i32) -> i32 {
    let mut i = 0;
    while i < n {
        if a[i as usize] != b[i as usize] {
            return 0;
        }
        i += 1;
    }
    1
}

fn sum(a: &[i32], n: i32) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += a[i as usize];
        i += 1;
    }
    s
}

fn main() {
    let mut x: [i32; 32] = [0; 32];
    let mut y: [i32; 32] = [0; 32];
    let mut y_snapshot: [i32; 32] = [0; 32];

    fill(&mut x, 32, 10);
    fill(&mut y, 32, 100);

    {
        let mut i = 0;
        while i < 32 {
            y_snapshot[i] = y[i];
            i += 1;
        }
    }

    {
        let gate4_val = unsafe { std::ptr::read_volatile(&GATE4) };
        
        if gate4_val != 0 {
            // copy_restrict(16, p2 + 8, q2 + 4) means copy from y[4..] to x[8..]
            let y_slice: Vec<i32> = y[4..20].to_vec();
            copy_restrict(16, &mut x[8..], &y_slice);

            {
                let mut expect: [i32; 16] = [0; 16];
                let mut i = 0;
                while i < 16 {
                    expect[i] = y_snapshot[4 + i];
                    i += 1;
                }
                if same(&x[8..], &expect, 16) == 0 {
                    std::process::exit(1);
                }
            }

            if same(&y, &y_snapshot, 32) == 0 {
                std::process::exit(2);
            }
        }

        // copy_restrict(8, p1 + 0, q1 + 24) means copy from y[24..] to x[0..]
        let y_slice: Vec<i32> = y[24..32].to_vec();
        copy_restrict(8, &mut x[0..], &y_slice);

        {
            let mut expect2: [i32; 8] = [0; 8];
            let mut i = 0;
            while i < 8 {
                expect2[i] = y_snapshot[24 + i];
                i += 1;
            }
            if same(&x[0..], &expect2, 8) == 0 {
                std::process::exit(3);
            }
        }
    }

    if same(&y, &y_snapshot, 32) == 0 {
        std::process::exit(4);
    }

    if sum(&x, 32) == 0 {
        std::process::exit(5);
    }

    std::process::exit(0);
}