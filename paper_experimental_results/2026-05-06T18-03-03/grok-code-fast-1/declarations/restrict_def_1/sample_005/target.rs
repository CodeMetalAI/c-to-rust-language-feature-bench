struct Bank {
    data: [i32; 8],
}

fn choose_view(u: &mut Bank, _which: i32) -> &mut [i32] {
    &mut u.data
}

fn fill(p: &mut [i32], base: i32) {
    for i in 0..8 {
        p[i] = base + i as i32;
    }
}

fn bump(p: &mut [i32], k: i32) {
    for i in 0..8 {
        p[i] += k;
    }
}

fn sum(p: &[i32]) -> i32 {
    p.iter().sum()
}

fn main() {
    let mut bank_a = Bank { data: [0; 8] };
    let mut bank_b = Bank { data: [0; 8] };
    let mut c = [0i32; 8];

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

    let expected_a: i32 = (101..=108).sum();
    if sum(a) != expected_a {
        std::process::exit(1);
    }

    let expected_b: i32 = (202..=209).sum();
    if sum(b) != expected_b {
        std::process::exit(2);
    }

    let expected_c: i32 = (303..=310).sum();
    if sum(&c) != expected_c {
        std::process::exit(3);
    }

    if bank_a.data[0] != 101 {
        std::process::exit(4);
    }

    if bank_b.data[7] != 209 {
        std::process::exit(5);
    }

    if c[0] != 303 || c[7] != 310 {
        std::process::exit(6);
    }

    std::process::exit(0);
}