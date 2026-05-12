fn main() {
    let mut n = 0;

    fn check(n: i32) {
        if n != 0 {
            panic!();
        }
    }

    check(match n {
        n_val if n_val > 0 => {
            n += 1;
            0
        }
        _ => 0
    });
    check(n);

    match n {
        n_val if n_val as f64 > 0.0 => {
            n += 1;
            n as i32
        }
        _ => 0,
    }
    check(n);

    let cn = 0;
    check(match cn {
        cn_val if cn_val == 0 => 0,
        _ => n += 1,
        n
    });
    check(n);

    check(match (n as i64) as i32 {
        cn_val if cn_val == 0 => 0,
        _ => n += 1,
        n
    });
    check(n);

    let a = [0; 1];
    let ca = [0; 1];
    check(match &a {
        ptr @ std::ptr::NonNull::new(0 as *mut i32) => 0,
        _ => {
            n += 1;
            0
        }
    });

    check(n);

    check(match &ca {
        ptr @ std::ptr::NonNull::new(0 as *const i32) => 0,
        _ => {
            n += 1;
            0
        }
    });


    check(n);

    fn f() {}
    check(match &f {
        fn_ptr @ _ => 0,
        _ => n += 1,
    });

    check(n);

    check(match &abort {
        fn_ptr @ _ => 0,
        _ => n += 1,
    });

    check(n);
    let mut s: i16 = 0;

    check(match s {
        _ if s == 0 => 0,
        _ => n += 1,
    });
    check(n);

    std::process::exit(0);
}
fn abort() {
    panic!();
}