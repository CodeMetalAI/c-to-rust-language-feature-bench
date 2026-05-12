use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
struct Array3D {
    data: Vec<Vec<Vec<usize>>>,
}

impl Array3D {
    fn new(nn: usize, mm: usize) -> Self {
        let mut data = vec![vec![vec![0; mm]; 6]; nn];
        for i in 0..nn {
            for j in 0..6 {
                for k in 0..mm {
                    data[i][j][k] = (i + 1) * 10000 + (j + 1) * 100 + (k + 1);
                }
            }
        }
        Self { data }
    }
}

fn check_a(nn: usize, mm: usize, a: &Array3D, p: &Array3D) -> bool {
    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                if a.data[i][j][k]!= p.data[j][k] + i {
                    return true;
                }
            }
        }
    }
    false
}

fn check_c(nn: usize, mm: usize, c: &Array3D, r: &Array3D) -> bool {
    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    if c.data[i][j][u][v]!= r.data[i][j][u][v] {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn fcompat() {
    let nn = 6;
    let mm = nn + 1;

    let a = Array3D::new(nn, mm);
    let mut p = a.data.as_mut_ptr();
    if check_a(nn, mm, &a, p) {
        atomic_gates.set(1, Ordering::SeqCst);
    }

    let c = Array3D::new(nn, mm);
    let mut r = c.data.as_mut_ptr();
    if check_c(nn, mm, &c, r) {
        atomic_gates.set(2, Ordering::SeqCst);
    }
}

static mut atomic_gates: AtomicUsize = AtomicUsize::new(0);

fn main() {
    unsafe {
        atomic_gates.store(0, Ordering::SeqCst);

        let n = 6;
        let m = n + 1;

        fcompat();

        if atomic_gates.load(Ordering::SeqCst)!= 0 {
            return 1;
        }
    }
    0
}