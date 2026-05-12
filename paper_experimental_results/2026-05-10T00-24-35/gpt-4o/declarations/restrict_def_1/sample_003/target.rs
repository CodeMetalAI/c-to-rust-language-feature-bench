// restrict_def_1.rs
#[derive(Copy, Clone)]
union Bank {
    left: [i32; 8],
    right: [i32; 8],
}

static mut BANK_A: Bank = Bank { left: [0; 8] };
static mut BANK_B: Bank = Bank { left: [0; 8] };

static mut C: [i32; 8] = [0; 8];

fn choose_view(u: &Bank, which: bool) -> &[i32; 8] {
    unsafe {
        if which {
            &u.right
        } else {
            &u.left
        }
    }
}

fn choose_view_mut(u: &mut Bank, which: bool) -> &mut [i32; 8] {
    unsafe {
        if which {
            &mut u.right
        } else {
            &mut u.left
        }
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
    let which_a = false;
    let which_b = true;

    let a = unsafe { choose_view_mut(&mut BANK_A, which_a) };
    let b = unsafe { choose_view_mut(&mut BANK_B, which_b) };

    unsafe {
        fill(a, 8, 100);
        fill(b, 8, 200);
        fill(&mut C, 8, 300);

        bump(a, 8, 1);
        bump(b, 8, 2);
        bump(&mut C, 8, 3);

        if sum(a, 8) != (101 + 102 + 103 + 104 + 105 + 106 + 107 + 108) {
            std::process::exit(1);
        }

        if sum(b, 8) != (202 + 203 + 204 + 205 + 206 + 207 + 208 + 209) {
            std::process::exit(2);
        }

        if sum(&C, 8) != (303 + 304 + 305 + 306 + 307 + 308 + 309 + 310) {
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

    std::process::exit(0);
}