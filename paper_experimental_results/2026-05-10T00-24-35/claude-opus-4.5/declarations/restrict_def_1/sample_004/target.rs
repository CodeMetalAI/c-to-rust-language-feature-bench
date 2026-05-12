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

    // We need to work with the union fields directly since we can't have multiple mutable references
    // Using a different approach: work with the arrays directly based on which view is selected
    
    let mut bank_a = Bank { left: [0; 8] };
    let mut bank_b = Bank { left: [0; 8] };
    let mut c = [0i32; 8];

    // a points to bank_a.left (which_a == 0)
    // b points to bank_b.right (which_b == 1)
    
    // fill a (bank_a.left), b (bank_b.right), c
    if which_a == 0 {
        fill(&mut bank_a.left, 8, 100);
    } else {
        fill(&mut bank_a.right, 8, 100);
    }
    
    if which_b == 0 {
        fill(&mut bank_b.left, 8, 200);
    } else {
        fill(&mut bank_b.right, 8, 200);
    }
    
    fill(&mut c, 8, 300);

    // bump a, b, c
    if which_a == 0 {
        bump(&mut bank_a.left, 8, 1);
    } else {
        bump(&mut bank_a.right, 8, 1);
    }
    
    if which_b == 0 {
        bump(&mut bank_b.left, 8, 2);
    } else {
        bump(&mut bank_b.right, 8, 2);
    }
    
    bump(&mut c, 8, 3);

    // sum checks
    let sum_a = if which_a == 0 {
        sum(&bank_a.left, 8)
    } else {
        sum(&bank_a.right, 8)
    };
    
    if sum_a != (101 + 102 + 103 + 104 + 105 + 106 + 107 + 108) {
        std::process::exit(1);
    }

    let sum_b = if which_b == 0 {
        sum(&bank_b.left, 8)
    } else {
        sum(&bank_b.right, 8)
    };
    
    if sum_b != (202 + 203 + 204 + 205 + 206 + 207 + 208 + 209) {
        std::process::exit(2);
    }

    if sum(&c, 8) != (303 + 304 + 305 + 306 + 307 + 308 + 309 + 310) {
        std::process::exit(3);
    }

    if bank_a.left[0] != 101 {
        std::process::exit(4);
    }

    if bank_b.right[7] != 209 {
        std::process::exit(5);
    }

    if c[0] != 303 || c[7] != 310 {
        std::process::exit(6);
    }

    std::process::exit(0);
}