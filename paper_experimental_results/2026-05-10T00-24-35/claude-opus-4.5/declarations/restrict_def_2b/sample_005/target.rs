use std::cell::Cell;

thread_local! {
    static GATE: Cell<i32> = Cell::new(1);
}

fn f(n: i32, p: &mut [i32], q: &[i32]) {
    let mut count = n;
    let mut pi = 0usize;
    let mut qi = 0usize;
    while count > 0 {
        count -= 1;
        p[pi] = q[qi];
        pi += 1;
        qi += 1;
    }
}

fn safe_move(n: i32, p: &mut [i32], p_start: usize, q: &[i32], q_start: usize) {
    if n <= 0 {
        return;
    }

    if p_start < q_start {
        let mut i = 0;
        while i < n as usize {
            p[p_start + i] = q[q_start + i];
            i += 1;
        }
    } else if p_start > q_start {
        let mut i = n as usize;
        while i > 0 {
            i -= 1;
            p[p_start + i] = q[q_start + i];
        }
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

fn call_f_checked(d: &mut [i32], n_total: i32, n: i32, p_offset: usize, q_offset: usize) {
    let ps = p_offset as i32;
    let qs = q_offset as i32;

    if ranges_overlap(n_total, ps, qs, n) {
        safe_move(n, d, p_offset, d, q_offset);
    } else {
        let n_usize = n as usize;
        if p_offset >= q_offset + n_usize || q_offset >= p_offset + n_usize {
            let (p_slice, q_slice) = if p_offset > q_offset {
                let (left, right) = d.split_at_mut(p_offset);
                (right, &left[q_offset..q_offset + n_usize])
            } else {
                let (left, right) = d.split_at_mut(q_offset);
                (&mut left[p_offset..p_offset + n_usize], &right[..n_usize])
            };
            f(n, p_slice, q_slice);
        } else {
            safe_move(n, d, p_offset, d, q_offset);
        }
    }
}

static mut D: [i32; 100] = [0; 100];

fn init(x: &mut [i32], n: usize) {
    let mut i = 0;
    while i < n {
        x[i] = (i as i32) * 13 + 5;
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

fn g_defined(d: &mut [i32]) {
    call_f_checked(d, 100, 50, 50, 0);
    call_f_checked(d, 100, 50, 1, 0);
}

fn main() {
    let mut d = [0i32; 100];
    let mut expect = [0i32; 100];

    init(&mut d, 100);
    init(&mut expect, 100);

    safe_move(50, &mut expect, 50, &expect.clone(), 0);
    safe_move(50, &mut expect, 1, &expect.clone(), 0);

    g_defined(&mut d);

    if !same(&d, &expect, 100) {
        std::process::exit(1);
    }

    std::process::exit(0);
}