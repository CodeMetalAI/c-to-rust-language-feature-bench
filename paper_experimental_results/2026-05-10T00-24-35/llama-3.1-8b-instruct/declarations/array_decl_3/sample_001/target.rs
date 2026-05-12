// Define external variables
extern crate core;

pub struct Array {
    data: Vec<Vec<Vec<i32>>>,
}

impl Array {
    pub fn new(n: usize, m: usize) -> Self {
        Array {
            data: vec![vec![vec![0; m]; 6]; n],
        }
    }

    pub fn set_value(&mut self, i: usize, j: usize, k: usize, value: i32) {
        self.data[i][j][k] = value;
    }
}

fn check_a(nn: usize, mm: usize, a: &Array, p: &Array) -> bool {
    for i in 0..nn {
        for j in 0..6 {
            for k in 0..mm {
                if a.data[i][j][k] != (*p).data[j][k] + (i as i32) {
                    return true;
                }
            }
        }
    }
    false
}

fn check_c(nn: usize, mm: usize, c: &Array, r: &Array) -> bool {
    for i in 0..nn {
        for j in 0..nn {
            for u in 0..6 {
                for v in 0..mm {
                    if c.data[i][j][u][v] != r.data[i][j][u][v] {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn fcompat() {
    let mut a = Array::new(6, 7);
    let mut c = Array::new(6, 7);
    let mut p = &a;
    let mut r = &c;

    for i in 0..6 {
        for j in 0..6 {
            for k in 0..7 {
                a.set_value(i, j, k, (i + 1) * 10000 + (j + 1) * 100 + (k + 1));
            }
        }
    }

    for i in 0..6 {
        for j in 0..6 {
            for u in 0..6 {
                for v in 0..7 {
                    c.set_value(i, j, u, v, (i + 1) * 1000000 + (j + 1) * 10000 + (u + 1) * 100 + (v + 1));
                }
            }
        }
    }

    if check_a(6, 7, &a, p) {
        return;
    }

    p = &c;
    if check_c(6, 7, &c, r) {
        return;
    }
}

fn main() {
    let mut gate = 0;

    gate = 0;
    fcompat();

    if gate != 0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}