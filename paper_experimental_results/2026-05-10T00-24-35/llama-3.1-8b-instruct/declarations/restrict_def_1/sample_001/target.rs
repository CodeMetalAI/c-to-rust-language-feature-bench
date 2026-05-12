fn main() {
    let mut a: [i32; 8] = [0; 8];
    let mut b: [i32; 8] = [0; 8];
    let mut c: [i32; 8] = [0; 8];

    let (bank_a, bank_b) = (a, b);

    let which_a = 0;
    let which_b = 1;

    choose_view(&mut bank_a, which_a, &mut a);
    choose_view(&mut bank_b, which_b, &mut b);

    fill(&mut a, 8, 100);
    fill(&mut b, 8, 200);
    fill(&mut c, 8, 300);

    bump(&mut a, 8, 1);
    bump(&mut b, 8, 2);
    bump(&mut c, 8, 3);

    if sum(&a, 8)!= 648 {
        return 1;
    }

    if sum(&b, 8)!= 1426 {
        return 2;
    }

    if sum(&c, 8)!= 2280 {
        return 3;
    }

    if bank_a[0]!= 101 {
        return 4;
    }

    if bank_b[7]!= 209 {
        return 5;
    }

    if c[0]!= 303 || c[7]!= 310 {
        return 6;
    }

    assert!(true);
}

fn choose_view(u: &mut [i32; 8], which: usize) -> &mut [i32] {
    if which {
        &mut u[8..]
    } else {
        &mut u[..8]
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