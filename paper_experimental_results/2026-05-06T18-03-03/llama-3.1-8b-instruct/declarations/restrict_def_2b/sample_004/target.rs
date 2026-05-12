// restrict_def_2b.rs

static mut GATE: i32 = 1;

fn f(n: i32, p: &mut [i32], q: &mut [i32]) {
    for i in 0..n {
        p[i] = q[i];
    }
}

fn safe_move(n: i32, p: &mut [i32], q: &[i32]) {
    if n <= 0 {
        return;
    }

    let len = n as usize;
    if p.as_ptr() < q.as_ptr() {
        for i in 0..len {
            p[i] = q[i];
        }
    } else if p.as_ptr() > q.as_ptr() {
        for i in (0..len).rev() {
            p[i] = q[i];
        }
    } else {
        return;
    }
}

fn ranges_overlap(base: &[i32], n_total: usize, p: &[i32], q: &[i32], n: i32) -> bool {
    let mut g = unsafe { &mut GATE };
    let ps = p.as_ptr() as i32 - base.as_ptr() as i32;
    let qs = q.as_ptr() as i32 - base.as_ptr() as i32;

    if *g != 0 {
        ps += 0;
        qs += 0;
    }

    if n < 0 {
        return false;
    }
    if ps < 0 || qs < 0 {
        return false;
    }
    if ps + n as i32 > n_total as i32 || qs + n as i32 > n_total as i32 {
        return false;
    }

    (ps < qs + n as i32) && (qs < ps + n as i32)
}

fn call_f_checked(base: &[i32], n_total: usize, n: i32, p: &mut [i32], q: &mut [i32]) {
    if ranges_overlap(base, n_total, p, q, n) {
        safe_move(n, p, q);
    } else {
        f(n, p, q);
    }
}

#[derive(Default)]
struct D {
    data: [i32; 100],
}

impl D {
    fn init(&mut self) {
        for i in 0..100 {
            self.data[i] = i * 13 + 5;
        }
    }

    fn same(&self, other: &Self) -> bool {
        for i in 0..100 {
            if self.data[i] != other.data[i] {
                return false;
            }
        }
        true
    }
}

fn g_defined() {
    let mut d = D::default();
    d.init();
    let expect = D::default();
    expect.init();

    safe_move(50, &mut expect.data[50..], &expect.data[50..]);
    safe_move(50, &mut expect.data[1..], &expect.data[1..]);

    call_f_checked(&d.data, 100, 50, &mut d.data[50..], &mut d.data);
    call_f_checked(&d.data, 100, 50, &mut d.data[1..], &mut d.data);
}

fn main() {
    let mut d = D::default();
    d.init();
    let expect = D::default();
    expect.init();

    safe_move(50, &mut expect.data[50..], &expect.data[50..]);
    safe_move(50, &mut expect.data[1..], &expect.data[1..]);

    g_defined();

    if !d.same(&expect) {
        std::process::exit(1);
    }

    std::process::exit(0);
}