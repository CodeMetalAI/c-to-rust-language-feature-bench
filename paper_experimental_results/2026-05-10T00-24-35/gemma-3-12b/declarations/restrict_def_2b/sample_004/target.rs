fn main() {
    let mut d = [0; 100];

    fn init(x: &mut [i32], n: usize) {
        let mut i = 0;
        while i < n {
            x[i] = i as i32 * 13 + 5;
            i += 1;
        }
    }

    fn same(x: &[i32], y: &[i32], n: usize) -> bool {
        let mut i = 0;
        while i < n {
            if x[i] != y[i] {
                return false;
            }
            i += 1;
        }
        true
    }

    init(&mut d, 100);
    let mut expect = [0; 100];
    init(&mut expect, 100);

    let mut expect_50_to_100 = expect[50..].to_vec();
    let mut expect_1_to_50 = expect[1..50].to_vec();

    let mut expect_50_to_100_copy = expect_50_to_100.clone();
    let mut expect_1_to_50_copy = expect_1_to_50.clone();

    // safe_move(50, expect + 50, expect);
    for i in 0..50 {
        expect_50_to_100_copy[i] = expect[i];
    }
    expect[50..].copy_from_slice(&expect_50_to_100_copy);


    // safe_move(50, expect + 1, expect);
    for i in 0..50 {
        expect_1_to_50_copy[i] = expect[i+1];
    }
    expect[1..50].copy_from_slice(&expect_1_to_50_copy);

    fn call_f_checked(base: &[i32], n_total: usize, n: usize, p: *mut i32, q: *mut i32) {
        let p_usize = p as usize - base.as_ptr() as usize;
        let q_usize = q as usize - base.as_ptr() as usize;

        if (p_usize < n_total && q_usize < n_total && p_usize + n <= n_total && q_usize + n <= n_total && p_usize < q_usize + n && q_usize < p_usize + n) {
            let p_slice = unsafe { std::slice::from_raw_parts_mut(p as *mut i32, n) };
            let q_slice = unsafe { std::slice::from_raw_parts(q as *const i32, n) };
            for i in 0..n {
                p_slice[i] = q_slice[i];
            }
        } else {
            fn f(n: usize, p: *mut i32, q: *mut i32) {
                let mut i = 0;
                while i < n {
                    unsafe { *p.add(i) = *q.add(i) };
                    i += 1;
                }
            }

            f(n, p, q);
        }
    }

    fn g_defined() {
        call_f_checked(&d, 100, 50, d.as_mut_ptr(), d.as_mut_ptr());
        call_f_checked(&d, 100, 50, d.as_mut_ptr().add(1), d.as_mut_ptr());
    }

    g_defined();

    if !same(&d, &expect, 100) {
        return 1;
    }

    0
}