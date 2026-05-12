union Bank {
    left: [i32; 8],
    right: [i32; 8],
}

static mut BANK_A: Bank = Bank { left: [0; 8] };
static mut BANK_B: Bank = Bank { left: [0; 8] };

static mut C: [i32; 8] = [0; 8];

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

    let mut bank_a = Bank { left: [0; 8] };
    let mut bank_b = Bank { left: [0; 8] };
    let mut c = [0i32; 8];

    let a: &mut [i32; 8] = if which_a != 0 {
        &mut bank_a.right
    } else {
        &mut bank_a.left
    };

    fill(a, 8, 100);

    let b: &mut [i32; 8] = if which_b != 0 {
        &mut bank_b.right
    } else {
        &mut bank_b.left
    };

    fill(b, 8, 200);
    fill(&mut c, 8, 300);

    let a: &mut [i32; 8] = if which_a != 0 {
        &mut bank_a.right
    } else {
        &mut bank_a.left
    };
    bump(a, 8, 1);

    let b: &mut [i32; 8] = if which_b != 0 {
        &mut bank_b.right
    } else {
        &mut bank_b.left
    };
    bump(b, 8, 2);
    bump(&mut c, 8, 3);

    let a: &[i32; 8] = if which_a != 0 {
        &bank_a.right
    } else {
        &bank_a.left
    };
    if sum(a, 8) != (101 + 102 + 103 + 104 + 105 + 106 + 107 + 108) {
        std::process::exit(1);
    }

    let b: &[i32; 8] = if which_b != 0 {
        &bank_b.right
    } else {
        &bank_b.left
    };
    if sum(b, 8) != (202 + 203 + 204 + 205 + 206 + 207 + 208 + 209) {
        std::process::exit(2);
    }

    if sum(&c, 8) != (303 + 304 + 305 + 306 + 307 + 308 + 309 + 310) {
        std::process::exit(3);
    }

    let bank_a_left = bank_a.left;
    if bank_a_left[0] != 101 {
        std::process::exit(4);
    }

    let bank_b_right = bank_b.right;
    if bank_b_right[7] != 209 {
        std::process::exit(5);
    }

    if c[0] != 303 || c[7] != 310 {
        std::process::exit(6);
    }

    std::process::exit(0);
}