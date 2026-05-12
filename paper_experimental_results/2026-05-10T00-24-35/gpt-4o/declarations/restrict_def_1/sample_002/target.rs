struct Bank {
    left: [i32; 8],
    right: [i32; 8],
}

impl Bank {
    fn new() -> Self {
        Self {
            left: [0; 8],
            right: [0; 8],
        }
    }
}

static mut C: [i32; 8] = [0; 8];

fn choose_view<'a>(bank: &'a mut Bank, which: i32) -> &'a mut [i32; 8] {
    if which != 0 {
        &mut bank.right
    } else {
        &mut bank.left
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
    let mut bank_a = Bank::new();
    let mut bank_b = Bank::new();

    let which_a = 0;
    let which_b = 1;

    let a = choose_view(&mut bank_a, which_a);
    let b = choose_view(&mut bank_b, which_b);

    fill(a, 8, 100);
    fill(b, 8, 200);

    unsafe {
        fill(&mut C, 8, 300);
    }

    bump(a, 8, 1);
    bump(b, 8, 2);

    unsafe {
        bump(&mut C, 8, 3);
    }

    if sum(a, 8) != (101 + 102 + 103 + 104 + 105 + 106 + 107 + 108) {
        std::process::exit(1);
    }

    if sum(b, 8) != (202 + 203 + 204 + 205 + 206 + 207 + 208 + 209) {
        std::process::exit(2);
    }

    unsafe {
        if sum(&C, 8) != (303 + 304 + 305 + 306 + 307 + 308 + 309 + 310) {
            std::process::exit(3);
        }
    }

    if bank_a.left[0] != 101 {
        std::process::exit(4);
    }

    if bank_b.right[7] != 209 {
        std::process::exit(5);
    }

    unsafe {
        if C[0] != 303 || C[7] != 310 {
            std::process::exit(6);
        }
    }
}