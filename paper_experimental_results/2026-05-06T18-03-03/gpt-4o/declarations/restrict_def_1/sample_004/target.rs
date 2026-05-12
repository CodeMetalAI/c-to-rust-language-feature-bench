struct Bank {
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

fn choose_view(bank: &Bank, which: bool) -> &[i32; 8] {
    if which {
        &bank.right
    } else {
        &bank.left
    }
}

fn choose_view_mut(bank: &mut Bank, which: bool) -> &mut [i32; 8] {
    if which {
        &mut bank.right
    } else {
        &mut bank.left
    }
}

fn fill(p: &mut [i32], base: i32) {
    for (i, elem) in p.iter_mut().enumerate() {
        *elem = base + i as i32;
    }
}

fn bump(p: &mut [i32], k: i32) {
    for elem in p.iter_mut() {
        *elem += k;
    }
}

fn sum(p: &[i32]) -> i32 {
    p.iter().sum()
}

fn main() {
    let which_a = false;
    let which_b = true;

    unsafe {
        let a = choose_view_mut(&mut BANK_A, which_a);
        let b = choose_view_mut(&mut BANK_B, which_b);

        fill(a, 100);
        fill(b, 200);
        fill(&mut C, 300);

        bump(a, 1);
        bump(b, 2);
        bump(&mut C, 3);

        if sum(a) != (101 + 102 + 103 + 104 + 105 + 106 + 107 + 108) {
            std::process::exit(1);
        }

        if sum(b) != (202 + 203 + 204 + 205 + 206 + 207 + 208 + 209) {
            std::process::exit(2);
        }

        if sum(&C) != (303 + 304 + 305 + 306 + 307 + 308 + 309 + 310) {
            std::process::exit(3);
        }

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