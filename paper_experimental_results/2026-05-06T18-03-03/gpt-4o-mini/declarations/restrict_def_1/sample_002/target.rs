struct Bank {
    left: [i32; 8],
    right: [i32; 8],
}

static mut BANK_A: Bank = Bank { left: [0; 8], right: [0; 8] };
static mut BANK_B: Bank = Bank { left: [0; 8], right: [0; 8] };

static mut C: [i32; 8] = [0; 8];

fn choose_view(bank: &Bank, which: usize) -> &[i32] {
    if which == 1 {
        &bank.right
    } else {
        &bank.left
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

fn main() -> i32 {
    let which_a = 0;
    let which_b = 1;

    let a: &mut [i32] = unsafe { &mut BANK_A.choose_view(which_a) };
    let b: &mut [i32] = unsafe { &mut BANK_B.choose_view(which_b) };

    fill(a, 8, 100);
    fill(b, 8, 200);
    fill(unsafe { &mut C }, 8, 300);

    bump(a, 8, 1);
    bump(b, 8, 2);
    bump(unsafe { &mut C }, 8, 3);

    if sum(a, 8) != (101 + 102 + 103 + 104 + 105 + 106 + 107 + 108) {
        return 1;
    }

    if sum(b, 8) != (202 + 203 + 204 + 205 + 206 + 207 + 208 + 209) {
        return 2;
    }

    if sum(unsafe { &C }, 8) != (303 + 304 + 305 + 306 + 307 + 308 + 309 + 310) {
        return 3;
    }

    if unsafe { BANK_A.left[0] } != 101 {
        return 4;
    }

    if unsafe { BANK_B.right[7] } != 209 {
        return 5;
    }

    if unsafe { C[0] } != 303 || unsafe { C[7] } != 310 {
        return 6;
    }

    0
}