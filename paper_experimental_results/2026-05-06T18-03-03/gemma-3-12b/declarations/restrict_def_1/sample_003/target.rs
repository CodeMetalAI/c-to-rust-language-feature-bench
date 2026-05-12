union Bank {
    left: [i32; 8],
    right: [i32; 8],
}

static mut BANK_A: Bank = Bank {
    left: [0; 8],
    right: [0; 8],
};

static mut BANK_B: Bank = Bank {
    left: [0; 8],
    right: [0; 8],
};

static mut C: [i32; 8] = [0; 8];

fn choose_view(u: &Bank, which: i32) -> &'static i32 {
    if which == 1 {
        &u.right[0]
    } else {
        &u.left[0]
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
    unsafe {
        let which_a = 0;
        let which_b = 1;

        let a = choose_view(&BANK_A, which_a);
        let b = choose_view(&BANK_B, which_b);

        fill(&mut BANK_A.left, 8, 100);
        fill(&mut BANK_B.right, 8, 200);
        fill(&mut C, 8, 300);

        bump(&mut BANK_A.left, 8, 1);
        bump(&mut BANK_B.right, 8, 2);
        bump(&mut C, 8, 3);

        if sum(&BANK_A.left, 8) != (101 + 102 + 103 + 104 + 105 + 106 + 107 + 108) {
            return 1;
        }

        if sum(&BANK_B.right, 8) != (202 + 203 + 204 + 205 + 206 + 207 + 208 + 209) {
            return 2;
        }

        if sum(&C, 8) != (303 + 304 + 305 + 306 + 307 + 308 + 309 + 310) {
            return 3;
        }

        if BANK_A.left[0] != 101 {
            return 4;
        }

        if BANK_B.right[7] != 209 {
            return 5;
        }

        if C[0] != 303 || C[7] != 310 {
            return 6;
        }

        0
    }
}