fn choose_view(u: &mut Bank, which: bool) -> &mut [i32] {
    if which {
        &mut u.right
    } else {
        &mut u.left
    }
}

fn fill(p: &mut [i32], base: i32) {
    for (i, x) in p.iter_mut().enumerate() {
        *x = base + i as i32;
    }
}

fn bump(p: &mut [i32], k: i32) {
    for x in p {
        *x += k;
    }
}

fn sum(p: &[i32]) -> i32 {
    p.iter().sum()
}

struct Bank {
    left: [i32; 8],
    right: [i32; 8],
}

fn main() {
    let mut bank_a = Bank {
        left: [0; 8],
        right: [0; 8],
    };

    let mut bank_b = Bank {
        left: [0; 8],
        right: [0; 8],
    };

    let mut c = [0; 8];

    let which_a = false;
    let which_b = true;

    let a = choose_view(&mut bank_a, which_a);
    let b = choose_view(&mut bank_b, which_b);

    fill(a, 100);
    fill(b, 200);
    fill(&mut c, 300);

    bump(a, 1);
    bump(b, 2);
    bump(&mut c, 3);

    if sum(a)!= 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 {
        std::process::exit(1);
    }

    if sum(b)!= 202 + 203 + 204 + 205 + 206 + 207 + 208 + 209 {
        std::process::exit(2);
    }

    if sum(&c)!= 303 + 304 + 305 + 306 + 307 + 308 + 309 + 310 {
        std::process::exit(3);
    }

    if bank_a.left[0]!= 101 {
        std::process::exit(4);
    }

    if bank_b.right[7]!= 209 {
        std::process::exit(5);
    }

    if c[0]!= 303 || c[7]!= 310 {
        std::process::exit(6);
    }
}