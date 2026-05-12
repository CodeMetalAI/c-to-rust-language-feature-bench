fn main() {
    static mut BANK_A: Bank = Bank {
        left: [0; 8],
        right: [0; 8],
    };
    static mut BANK_B: Bank = Bank {
        left: [0; 8],
        right: [0; 8],
    };
    static mut C: [i32; 8] = [0; 8];

    let which_a = 0;
    let which_b = 1;

    let a = unsafe { choose_view(&mut BANK_A, which_a) };
    let b = unsafe { choose_view(&mut BANK_B, which_b) };
    let c = unsafe { &mut C };

    fill(a, 100);
    fill(b, 200);
    fill(c, 300);

    bump(a, 1);
    bump(b, 2);
    bump(c, 3);

    if sum(a) != (101 + 102 + 103 + 104 + 105 + 106 + 107 + 108) {
        std::process::exit(1);
    }

    if sum(b) != (202 + 203 + 204 + 205 + 206 + 207 + 208 + 209) {
        std::process::exit(2);
    }

    if sum(c) != (303 + 304 + 305 + 306 + 307 + 308 + 309 + 310) {
        std::process::exit(3);
    }

    unsafe {
        if BANK_A.left[0] != 101 {
            std::process::exit(4);
        }

        if BANK_B.right[7] != 209 {
            std::process::exit(5);
        }

        if C[0] != 303 || C[7] != 310 {
            std::process::exit(6);
        }
    }
}

union Bank {
    left: [i32; 8],
    right: [i32; 8],
}

fn choose_view(bank: &mut Bank, which: i32) -> &mut [i32; 8] {
    unsafe {
        if which != 0 {
            &mut bank.right
        } else {
            &mut bank.left
        }
    }
}

fn fill(p: &mut [i32; 8], base: i32) {
    for i in 0..8 {
        p[i] = base + i as i32;
    }
}

fn bump(p: &mut [i32; 8], k: i32) {
    for i in 0..8 {
        p[i] += k;
    }
}

fn sum(p: &[i32; 8]) -> i32 {
    p.iter().sum()
}