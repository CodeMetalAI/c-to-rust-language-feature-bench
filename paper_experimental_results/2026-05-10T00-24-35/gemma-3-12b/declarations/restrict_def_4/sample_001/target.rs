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

    {
        let mut p1 = &mut x[0..32];
        let mut q1 = &mut y[0..32];

        if gate4() {
            let mut p2 = &mut x[0..32];
            let mut q2 = &mut y[0..32];

            unsafe {
                copy_restrict(16, &mut *p2.get_unchecked(8..).unwrap(), &mut *q2.get_unchecked(4..).unwrap());
            }

            {
                let expect = [0; 16];
                let mut i = 0;
                while i < 16 {
                    expect[i] = y_snapshot[4 + i];
                    i += 1;
                }
                if !same(&x[8..], &expect) {
                    return 1;
                }
            }

            if !same(&y[..], &y_snapshot) {
                return 2;
            }
        }

        unsafe {
            copy_restrict(8, &mut *p1.get_unchecked(0..).unwrap(), &mut *q1.get_unchecked(24..).unwrap());
        }

        {
            let expect2 = [0; 8];
            let mut i = 0;
            while i < 8 {
                expect2[i] = y_snapshot[24 + i];
                i += 1;
            }
            if !same(&x[0..], &expect2) {
                return 3;
            }
        }
    }

    if !same(&y[..], &y_snapshot) {
        return 4;
    }

    if sum(&x) == 0 {
        return 5;
    }

    0
}

volatile_static!(gate4: i32 = 1);

fn gate4() -> i32 {
    gate4
}

fn copy_restrict(n: usize, p: *mut i32, q: *mut i32) {
    unsafe {
        while n > 0 {
            *p = *q;
            p = p.add(1);
            q = q.add(1);
            n -= 1;
        }
    }
}

fn fill(p: &mut [i32], n: usize, base: i32) {
    for i in 0..n {
        p[i] = base + i * 5 + 1;
    }
}

fn same(a: &[i32], b: &[i32]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}

fn sum(a: &[i32]) -> i32 {
    let mut s = 0;
    for &x in a {
        s += x;
    }
    s
}

macro_rules! volatile_static {
    ($name:ident: $t:ty = $val:expr) => {
        static $name: std::sync::Mutex<std::cell::RefCell<$t>> = {
            let initial_value = $val;
            std::sync::Mutex::new(std::cell::RefCell::new(initial_value))
        };
    };
}