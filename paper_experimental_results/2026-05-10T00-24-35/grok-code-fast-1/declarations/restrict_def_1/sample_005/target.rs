#[derive(Clone, Copy)]
struct Bank {
    arr: [i32; 8],
}

fn choose_view(bank: &mut Bank, _which: i32) -> &mut [i32] {
    &mut bank.arr
}

fn fill(arr: &mut [i32], base: i32) {
    for i in 0..arr.len() {
        arr[i] = base + i as i32;
    }
}

fn bump(arr: &mut [i32], k: i32) {
    for val in arr.iter_mut() {
        *val += k;
    }
}

fn sum(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

fn main() {
    let mut bank_a = Bank { arr: [0; 8] };
    let mut bank_b = Bank { arr: [0; 8] };
    let mut c: [i32; 8] = [0; 8];

    let which_a = 0;
    let which_b = 1;

    let a = choose_view(&mut bank_a, which_a);
    let b = choose_view(&mut bank_b, which_b);

    fill(a, 100);
    fill(b, 200);
    fill(&mut c, 300);

    bump(a, 1);
    bump(b, 2);
    bump(&mut c, 3);

    if sum(a) != 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 {
        std::process::exit(1);
    }

    if sum(b) != 202 + 203 + 204 + 205 + 206 + 207 + 208 + 209 {
        std::process::exit(2);
    }

    if sum(&c) != 303 + 304 + 305 + 306 + 307 + 308 + 309 + 310 {
        std::process::exit(3);
    }

    if bank_a.arr[0] != 101 {
        std::process::exit(4);
    }

    if bank_b.arr[7] != 209 {
        std::process::exit(5);
    }

    if c[0] != 303 || c[7] != 310 {
        std::process::exit(6);
    }

    std::process::exit(0);
}