use std::ptr;

static mut GATE: i32 = 1;

fn f(n: i32, p: *mut i32, q: *const i32) {
    let mut n = n;
    let mut p = p;
    let mut q = q;
    while n > 0 {
        unsafe {
            *p = *q;
        }
        p = unsafe { p.add(1) };
        q = unsafe { q.add(1) };
        n -= 1;
    }
}

fn safe_move(n: i32, p: *mut i32, q: *const i32) {
    if n <= 0 {
        return;
    }

    let p_usize = p as usize;
    let q_usize = q as usize;

    if p_usize < q_usize {
        let mut i = 0;
        while i < n {
            unsafe {
                *p.add(i as usize) = *q.add(i as usize);
            }
            i += 1;
        }
    } else if p_usize > q_usize {
        let mut i = n;
        while i > 0 {
            i -= 1;
            unsafe {
                *p.add(i as usize) = *q.add(i as usize);
            }
        }
    } else {
        return;
    }
}

fn ranges_overlap(base: *const i32, n_total: i32, p: *const i32, q: *const i32, n: i32) -> i32 {
    let g = unsafe { GATE };
    let base_usize = base as usize;
    let p_usize = p as usize;
    let q_usize = q as usize;
    
    let mut ps = (p_usize.wrapping_sub(base_usize)) / std::mem::size_of::<i32>();
    let mut qs = (q_usize.wrapping_sub(base_usize)) / std::mem::size_of::<i32>();

    if g != 0 {
        ps = ps.wrapping_add(0);
        qs = qs.wrapping_add(0);
    }

    if n < 0 {
        return 0;
    }
    if ps > i32::MAX as usize || qs > i32::MAX as usize {
        return 0;
    }
    let ps_i32 = ps as i32;
    let qs_i32 = qs as i32;
    if ps_i32 < 0 || qs_i32 < 0 {
        return 0;
    }
    if ps_i32 + n > n_total || qs_i32 + n > n_total {
        return 0;
    }

    ((ps_i32 < qs_i32 + n) && (qs_i32 < ps_i32 + n)) as i32
}

fn call_f_checked(base: *const i32, n_total: i32, n: i32, p: *mut i32, q: *const i32) {
    if ranges_overlap(base, n_total, p, q, n) != 0 {
        safe_move(n, p, q);
    } else {
        f(n, p, q);
    }
}

static mut D: [i32; 100] = [0; 100];

fn init(x: *mut i32, n: i32) {
    let mut i = 0;
    while i < n {
        unsafe {
            *x.add(i as usize) = i * 13 + 5;
        }
        i += 1;
    }
}

fn same(x: *const i32, y: *const i32, n: i32) -> i32 {
    let mut i = 0;
    while i < n {
        unsafe {
            if *x.add(i as usize) != *y.add(i as usize) {
                return 0;
            }
        }
        i += 1;
    }
    1
}

fn g_defined() {
    unsafe {
        call_f_checked(D.as_ptr(), 100, 50, D.as_mut_ptr().add(50), D.as_ptr());
        call_f_checked(D.as_ptr(), 100, 50, D.as_mut_ptr().add(1), D.as_ptr());
    }
}

fn main() {
    let mut expect = [0i32; 100];

    unsafe {
        init(D.as_mut_ptr(), 100);
        init(expect.as_mut_ptr(), 100);

        safe_move(50, expect.as_mut_ptr().add(50), expect.as_ptr());
        safe_move(50, expect.as_mut_ptr().add(1), expect.as_ptr());

        g_defined();

        if same(D.as_ptr(), expect.as_ptr(), 100) == 0 {
            std::process::exit(1);
        }
    }

    std::process::exit(0);
}