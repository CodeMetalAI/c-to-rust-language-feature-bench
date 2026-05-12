fn f(n: isize, p: *mut i32, q: *const i32) {
    let mut p = p;
    let mut q = q;
    let mut n = n;
    while n > 0 {
        unsafe {
            *p = *q;
        }
        p = unsafe { p.add(1) };
        q = unsafe { q.add(1) };
        n -= 1;
    }
}

static mut GATE: i32 = 1;

fn safe_move(n: isize, p: *mut i32, q: *const i32) {
    if n <= 0 {
        return;
    }
    let p = p as usize;
    let q = q as usize;
    if p < q {
        let mut i = 0;
        while i < n {
            unsafe {
                *((p as *mut i32).add(i as usize)) = *((q as *const i32).add(i as usize));
            }
            i += 1;
        }
    } else if p > q {
        let mut i = n;
        while i > 0 {
            i -= 1;
            unsafe {
                *((p as *mut i32).add(i as usize)) = *((q as *const i32).add(i as usize));
            }
        }
    } else {
        return;
    }
}

fn ranges_overlap(base: *const i32, n_total: isize, p: *const i32, q: *const i32, n: isize) -> i32 {
    let g = unsafe { GATE };
    let base = base as isize;
    let ps = (p as isize - base) / std::mem::size_of::<i32>() as isize;
    let qs = (q as isize - base) / std::mem::size_of::<i32>() as isize;
    let ps = if g != 0 { ps + 0 } else { ps };
    let qs = if g != 0 { qs + 0 } else { qs };
    if n < 0 {
        return 0;
    }
    if ps < 0 || qs < 0 {
        return 0;
    }
    if ps + n > n_total || qs + n > n_total {
        return 0;
    }
    ((ps < qs + n) && (qs < ps + n)) as i32
}

fn call_f_checked(base: *const i32, n_total: isize, n: isize, p: *mut i32, q: *const i32) {
    if ranges_overlap(base, n_total, p, q, n) != 0 {
        safe_move(n, p, q);
    } else {
        f(n, p, q);
    }
}

static mut D: [i32; 100] = [0; 100];

fn init(x: *mut i32, n: isize) {
    let mut i = 0;
    while i < n {
        unsafe {
            *x.add(i as usize) = i * 13 + 5;
        }
        i += 1;
    }
}

fn same(x: *const i32, y: *const i32, n: isize) -> i32 {
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
    let mut expect = [0; 100];
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