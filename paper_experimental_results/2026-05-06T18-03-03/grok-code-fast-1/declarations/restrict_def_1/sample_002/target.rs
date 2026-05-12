use std::process;

struct Bank {
    data: [i32; 8],
}

impl Bank {
    fn choose_view(&mut self, _which: i32) -> &mut [i32] {
        &mut self.data
    }
}

fn fill(p: &mut [i32], n: usize, base: i32) {
    for i in 0..n {
        p[i] = base + i as i32;
    }
}

fn bump(p: &mut [i32], n: usize, k: i32) {
    for i in 0..n {
        p[i] += k;
    }
}

fn sum(p: &[i32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += p[i];
    }
    s
}

fn main() {
    let mut bank_a = Bank { data: [0; 8] };
    let mut bank_b = Bank { data: [0; 8] };
    let mut c = [0i32; 8];

    let which_a = 0;
    let which_b = 1;

    let a = bank_a.choose_view(which_a);
    let b = bank_b.choose_view(which_b);

    fill(a, 8, 100);
    fill(b, 8, 200);
    fill(&mut c, 8, 300);

    bump(a, 8, 1);
    bump(b, 8, 2);
    bump(&mut c, 8, 3);

    if sum(a, 8) != (101 + 102 + 103 + 104 + 105 + 106 + 107 + 108) {
        process::exit(1);
    }

    if sum(b, 8) != (202 + 203 + 204 + 205 + 206 + 207 + 208 + 209) {
        process::exit(2);
    }

    if sum(&c, 8) != (303 + 304 + 305 + 306 + 307 + 308 + 309 + 310) {
        process::exit(3);
    }

    if bank_a.data[0] != 101 {
        process::exit(4);
    }

    if bank_b.data[7] != 209 {
        process::exit(5);
    }

    if c[0] != 303 || c[7] != 310 {
        process::exit(6);
    }

    process::exit(0);
}