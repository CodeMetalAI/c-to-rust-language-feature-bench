union Bank {
    left: [i32; 8],
    right: [i32; 8],
}

static mut BANK_A: Bank = Bank { left: [0; 8] };
static mut BANK_B: Bank = Bank { left: [0; 8] };
static mut C: [i32; 8] = [0; 8];

fn choose_view(u: &mut Bank, which: i32) -> &mut [i32; 8] {
    if which != 0 {
        unsafe { &mut u.right }
    } else {
        unsafe { &mut u.left }
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

    let a: &mut [i32; 8];
    let b: &mut [i32; 8];
    unsafe {
        a = choose_view(&mut BANK_A, which_a);
        b = choose_view(&mut BANK_B, which_b);
    }

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

    std::process::exit(0);
}