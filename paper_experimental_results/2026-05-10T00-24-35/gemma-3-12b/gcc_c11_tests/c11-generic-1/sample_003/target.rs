fn main() {
    let mut n = 0;

    fn check(n: i32) {
        if n != 0 {
            panic!("Aborted");
        }
    }

    check(match n {
        x if x == 0 => 0,
        _ => unreachable!(),
    });
    check(n);

    check(match n {
        x if x as f64 == x as i32 as f64 => {
            n += 1;
            n
        }
        _ => 0,
    });
    check(n);

    let cn = 0;
    check(match cn {
        x if x == 0 => 0,
        _ => n += 1,
        _ => 0,
    });
    check(n);

    check(match (cn as i32) {
        x if x == 0 => 0,
        _ => n += 1,
        _ => 0,
    });
    check(n);

    let mut a = [0; 1];
    let ca = [0; 1];
    check(match &a {
        ptr => {
            if ptr as *const i32 == ptr as *const i32 {
                0
            } else {
                n += 1;
                n
            }
        }
        _ => 0,
    });
    check(n);
    check(match &ca {
        ptr => {
            if ptr as *const i32 == ptr as *mut i32 {
                0
            } else {
                n += 1;
                n
            }
        }
        _ => 0,
    });
    check(n);

    fn f() {}
    check(match &f {
        func_ptr => {
            if func_ptr as *const fn() == func_ptr as *const fn() {
                0
            } else {
                n += 1;
                n
            }
        }
        _ => 0,
    });
    check(n);

    check(match &abort {
        func_ptr => {
            if func_ptr as *const fn() == func_ptr as *const fn() {
                0
            } else {
                n += 1;
                n
            }
        }
        _ => 0,
    });
    check(n);

    let mut s: i16 = 0;
    check(match s {
        x if x == 0 => 0,
        _ => n += 1,
        _ => 0,
    });
    check(n);
}