const GATE: i32 = 1;

fn f(arr: &mut [i32], p_idx: usize, q_idx: usize, n: usize) {
    for i in 0..n {
        arr[p_idx + i] = arr[q_idx + i];
    }
}

fn safe_move(arr: &mut [i32], p_idx: usize, q_idx: usize, n: usize) {
    if n == 0 {
        return;
    }
    if p_idx < q_idx {
        for i in 0..n {
            arr[p_idx + i] = arr[q_idx + i];
        }
    } else if p_idx > q_idx {
        for i in (0..n).rev() {
            arr[p_idx + i] = arr[q_idx + i];
        }
    }
}

fn ranges_overlap(n_total: usize, p_idx: usize, q_idx: usize, n: i32) -> bool {
    let g = GATE;
    let mut ps = p_idx as i32;
    let mut qs = q_idx as i32;
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
    let n_total_i32 = n_total as i32;
    if ps + n > n_total_i32 || qs + n > n_total_i32 {
        return false;
    }
    (ps < qs + n) && (qs < ps + n)
}

fn call_f_checked(arr: &mut [i32], n_total: usize, n: usize, p_idx: usize, q_idx: usize) {
    if ranges_overlap(n_total, p_idx, q_idx, n as i32) {
        safe_move(arr, p_idx, q_idx, n);
    } else {
        f(arr, p_idx, q_idx, n);
    }
}

fn init(x: &mut [i32]) {
    for i in 0..x.len() {
        x[i] = (i as i32) * 13 + 5;
    }
}

fn same(x: &[i32], y: &[i32]) -> bool {
    x == y
}

fn g_defined(d: &mut [i32]) {
    call_f_checked(d, 100, 50, 50, 0);
    call_f_checked(d, 100, 50, 1, 0);
}

fn main() -> i32 {
    let mut d = [0i32; 100];
    let mut expect = [0i32; 100];
    init(&mut d);
    init(&mut expect);
    safe_move(&mut expect, 50, 0, 50);
    safe_move(&mut expect, 1, 0, 50);
    g_defined(&mut d);
    if !same(&d, &expect) {
        return 1;
    }
    0
}