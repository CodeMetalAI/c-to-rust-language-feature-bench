use std::process::exit;

struct Bank {
    left: [i32; 8],
    right: [i32; 8],
}

fn choose_view(u: &mut Bank, which: i32) -> &mut [i32; 8] {
    if which != 0 {
        &mut u.right
    } else {
        &mut u.left
    }
}

fn fill(p: &mut [i32], base: i32) {
    for i in 0..p.len() {
        p[i] = base + i as i32;
    }
}

fn bump(p: &mut [i32], k: i32) {
    for i in 0..p.len() {
        p[i] += k;
    }
}

fn sum(p: &[i32]) -> i32 {
    let mut s = 0;
    for &v in p.iter() {
        s += v;
    }
    s
}

fn main() {
    let mut bank_a = Bank {
        left: [0; 8],
        right: [0; 8],
    };
    let mut bank_b = Bank {
        left: [0; 8],
        right: [0; 8],
    };
    let mut c = [0i32; 8];

    let which_a = 0;
    let which_b = 1;

    let (sum_a, sum_b, sum_c) = {
        let a = choose_view(&mut bank_a, which_a);
        let b = choose_view(&mut bank_b, which_b);

        fill(a, 100);
        fill(b, 200);
        fill(&mut c, 300);

        bump(a, 1);
        bump(b, 2);
        bump(&mut c, 3);

        (sum(&*a), sum(&*b), sum(&c))
    };

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