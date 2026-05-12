fn choose_view(bank: &mut Bank, side: usize) -> &mut [i32; 8] {
    match side {
        1 => &mut bank.right,
        _ => &mut bank.left,
    }
}

fn fill(array: &mut [i32], base: i32) {
    for (i, item) in array.iter_mut().enumerate() {
        *item = base + i as i32;
    }
}

fn bump(array: &mut [i32], k: i32) {
    for item in array.iter_mut() {
        *item += k;
    }
}

fn sum(array: &[i32]) -> i32 {
    array.iter().sum()
}

union Bank {
    left: [i32; 8],
    right: [i32; 8],
}

fn main() {
    let which_a = 0;
    let which_b = 1;

    // Initialize the banks
    let mut bank_a = Bank { left: [0; 8] };
    let mut bank_b = Bank { left: [0; 8] };

    // Mutable references to Bank::left and Bank::right
    let a = choose_view(&mut bank_a, which_a);
    let b = choose_view(&mut bank_b, which_b);

    // Global array `c`
    let mut c = [0; 8];

    fill(a, 100);
    fill(b, 200);
    fill(&mut c, 300);

    bump(a, 1);
    bump(b, 2);
    bump(&mut c, 3);

    if sum(a) != (101 + 102 + 103 + 104 + 105 + 106 + 107 + 108) {
        std::process::exit(1);
    }

    if sum(b) != (202 + 203 + 204 + 205 + 206 + 207 + 208 + 209) {
        std::process::exit(2);
    }

    if sum(&c) != (303 + 304 + 305 + 306 + 307 + 308 + 309 + 310) {
        std::process::exit(3);
    }

    unsafe {
        if bank_a.left[0] != 101 {
            std::process::exit(4);
        }

        if bank_b.right[7] != 209 {
            std::process::exit(5);
        }
    }

    if c[0] != 303 || c[7] != 310 {
        std::process::exit(6);
    }
}