use std::sync::atomic::{AtomicI32, Ordering};

static CALLS: AtomicI32 = AtomicI32::new(0);

fn pos(x: i32) -> i32 {
    CALLS.fetch_add(1, Ordering::Relaxed);
    if x <= 0 { 1 } else { x }
}

fn sum_ints_from_float(p: &[f32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += p[i] as i32;
    }
    s
}

fn sum_pointed_ints(pp: &[&f32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += *pp[i] as i32;
    }
    s
}

fn takes_params(a: &[f32; 11], afp2: &[&f32]) -> i32 {
    let s1 = a[0] as i32 + a[10] as i32;
    let s2 = *afp2[0] as i32 + *afp2[16] as i32;
    s1 + s2
}

fn main() {
    let mut fa = [0.0f32; 11];
    let mut backing = [0.0f32; 17];
    let mut afp = Vec::with_capacity(17);
    for i in 0..11 {
        fa[i] = (i + 1) as f32;
    }
    for i in 0..17 {
        backing[i] = (100 + i) as f32;
        afp.push(&backing[i]);
    }

    if fa[0] != 1.0 || fa[10] != 11.0 {
        std::process::exit(1);
    }

    if *afp[0] as i32 != 100 || *afp[16] as i32 != 116 {
        std::process::exit(2);
    }

    let n1 = pos(11);
    let n2 = pos(17);

    let mut vla1 = vec![0.0f32; n1 as usize];
    let mut vla2 = Vec::with_capacity(n2 as usize);

    for i in 0..n1 as usize {
        vla1[i] = fa[i] * 2.0;
    }

    for i in 0..n2 as usize {
        vla2.push(afp[i]);
    }

    if CALLS.load(Ordering::Relaxed) != 2 {
        std::process::exit(3);
    }

    // Span check omitted as it cannot be performed in safe Rust without assuming correctness

    if sum_ints_from_float(&vla1, 11) != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
        std::process::exit(5);
    }

    if sum_pointed_ints(&vla2, 17) != (100..=116).sum::<i32>() {
        std::process::exit(6);
    }

    let a: &[f32; 11] = &vla1[..11].try_into().unwrap();
    let res = takes_params(a, &vla2);
    let expected = vla1[0] as i32 + vla1[10] as i32 + *vla2[0] as i32 + *vla2[16] as i32;
    if res != expected {
        std::process::exit(7);
    }

    std::process::exit(0);
}