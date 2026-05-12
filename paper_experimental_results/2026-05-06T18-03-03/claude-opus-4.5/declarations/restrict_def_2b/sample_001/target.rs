use std::cell::Cell;

thread_local! {
    static GATE: Cell<i32> = Cell::new(1);
}

fn f(n: i32, p: &mut [i32], q: &[i32]) {
    let mut i = 0;
    let mut remaining = n;
    while remaining > 0 {
        p[i] = q[i];
        i += 1;
        remaining -= 1;
    }
}

fn safe_move(n: i32, p: &mut [i32], q: &[i32]) {
    if n <= 0 {
        return;
    }

    let n = n as usize;
    for i in 0..n {
        p[i] = q[i];
    }
}

fn safe_move_backward(n: i32, p: &mut [i32], q: &[i32]) {
    if n <= 0 {
        return;
    }

    let mut i = n as usize;
    while i > 0 {
        i -= 1;
        p[i] = q[i];
    }
}

fn ranges_overlap(n_total: i32, ps: i32, qs: i32, n: i32) -> bool {
    let g = GATE.with(|gate| gate.get());
    
    let mut ps = ps;
    let mut qs = qs;
    
    if g != 0 {
        ps += 0;
        qs += 0;
    }

    if n < 0 {
        return false;
    }
    if ps < 0 || qs < 0 {
        return false;
    }
    if ps + n > n_total || qs + n > n_total {
        return false;
    }

    (ps < qs + n) && (qs < ps + n)
}

fn call_f_checked(d: &mut [i32], n_total: i32, n: i32, p_start: usize, q_start: usize) {
    let ps = p_start as i32;
    let qs = q_start as i32;
    
    if ranges_overlap(n_total, ps, qs, n) {
        if p_start < q_start {
            let (left, right) = d.split_at_mut(q_start);
            safe_move(n, &mut left[p_start..], right);
        } else if p_start > q_start {
            let n_usize = n as usize;
            let temp: Vec<i32> = d[q_start..q_start + n_usize].to_vec();
            safe_move_backward(n, &mut d[p_start..], &temp);
        }
    } else {
        let n_usize = n as usize;
        let temp: Vec<i32> = d[q_start..q_start + n_usize].to_vec();
        f(n, &mut d[p_start..], &temp);
    }
}

static mut D: [i32; 100] = [0; 100];

fn init(x: &mut [i32], n: i32) {
    let mut i = 0;
    while i < n as usize {
        x[i] = (i as i32) * 13 + 5;
        i += 1;
    }
}

fn same(x: &[i32], y: &[i32], n: i32) -> bool {
    let mut i = 0;
    while i < n as usize {
        if x[i] != y[i] {
            return false;
        }
        i += 1;
    }
    true
}

fn g_defined(d: &mut [i32]) {
    call_f_checked(d, 100, 50, 50, 0);
    call_f_checked(d, 100, 50, 1, 0);
}

fn main() {
    let mut d = [0i32; 100];
    let mut expect = [0i32; 100];

    init(&mut d, 100);
    init(&mut expect, 100);

    // safe_move(50, expect + 50, expect) - copy first 50 elements to position 50
    {
        let temp: Vec<i32> = expect[0..50].to_vec();
        safe_move(50, &mut expect[50..], &temp);
    }
    
    // safe_move(50, expect + 1, expect) - copy first 50 elements to position 1
    {
        let temp: Vec<i32> = expect[0..50].to_vec();
        for i in (0..50).rev() {
            expect[i + 1] = temp[i];
        }
    }

    g_defined(&mut d);

    if !same(&d, &expect, 100) {
        std::process::exit(1);
    }

    std::process::exit(0);
}