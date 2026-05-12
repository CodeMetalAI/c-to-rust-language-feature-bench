use std::process::exit;

struct Bank {
    left: [i32; 8],
    right: [i32; 8],
}

impl Bank {
    fn new() -> Self {
        Bank {
            left: [0; 8],
            right: [0; 8],
        }
    }
}

fn choose_view(u: &mut Bank, which: i32) -> &mut [i32; 8] {
    if which != 0 {
        &mut u.right
    } else {
        &mut u.left
    }
}

fn fill(p: &mut [i32], n: usize, base: i32) {
    let mut i = 0;
    while i < n {
        p[i] = base + i as i32;
        i += 1;
    }
}

fn bump(p: &mut [i32], n: usize, k: i32) {
    let mut i = 0;
    while i < n {
        p[i] += k;
        i += 1;
    }
}

fn sum(p: &[i32], n: usize) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += p[i];
        i += 1;
    }
    s
}

fn main() {
    let which_a = 0;
    let which_b = 1;

    let mut bank_a = Bank::new();
    let mut bank_b = Bank::new();
    let mut c = [0i32; 8];

    let sum_a: i32;
    {
        let a = choose_view(&mut bank_a, which_a);
        fill(a, 8, 100);
        bump(a, 8, 1);
        sum_a = sum(a, 8);
    }

    let sum_b: i32;
    {
        let b = choose_view(&mut bank_b, which_b);
        fill(b, 8, 200);
        bump(b, 8, 2);
        sum_b = sum(b, 8);
    }

    fill(&mut c, 8, 300);
    bump(&mut c, 8, 3);
    let sum_c = sum(&c, 8);

    if sum_a != (101 + 102 + 103 + 104 + 105 + 106 + 107 + 108) {
        exit(1);
    }

    if sum_b != (202 + 203 + 204 + 205 + 206 + 207 + 208 + 209) {
        exit(2);
    }

    if sum_c != (303 + 304 + 305 + 306 + 307 + 308 + 309 + 310) {
        exit(3);
    }

    if bank_a.left[0] != 101 {
        exit(4);
    }

    if bank_b.right[7] != 209 {
        exit(5);
    }

    if c[0] != 303 || c[7] != 310 {
        exit(6);
    }

    exit(0);
}