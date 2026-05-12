use std::sync::atomic::{AtomicI32, Ordering};

static calls: AtomicI32 = AtomicI32::new(0);

fn pos(x: i32) -> i32 {
    calls.fetch_add(1, Ordering::SeqCst);
    if x <= 0 { 1 } else { x }
}

fn sum_ints_from_float(p: &[f32]) -> i32 {
    p.iter().map(|&x| x as i32).sum()
}

fn sum_pointed_ints(pp: &[Option<&f32>]) -> i32 {
    pp.iter().map(|&opt| *opt.unwrap() as i32).sum()
}

fn takes_params(a: &[f32; 11], afp2: &[Option<&f32>; 17]) -> i32 {
    let s1 = a[0] as i32 + a[10] as i32;
    let s2 = *afp2[0].unwrap() as i32 + *afp2[16].unwrap() as i32;
    s1 + s2
}

fn main() -> i32 {
    let mut fa = [0.0f32; 11];
    let mut backing = [0.0f32; 17];
    let mut afp = [None; 17];

    for i in 0..11 {
        fa[i] = (i + 1) as f32;
    }
    for i in 0..17 {
        backing[i] = (100 + i) as f32;
        afp[i] = Some(&backing[i]);
    }

    if fa[0] != 1.0 || fa[10] != 11.0 {
        return 1;
    }

    if *afp[0].unwrap() as i32 != 100 || *afp[16].unwrap() as i32 != 116 {
        return 2;
    }

    let n1 = pos(11);
    let n2 = pos(17);

    let mut vla1 = [0.0f32; 11];
    for i in 0..11 {
        vla1[i] = fa[i] * 2.0;
    }

    let mut vla2 = [None; 17];
    for i in 0..17 {
        vla2[i] = afp[i];
    }

    if calls.load(Ordering::SeqCst) != 2 {
        return 3;
    }

    // Span check omitted as it requires unsafe code, but in safe Rust Vec/array is contiguous

    if sum_ints_from_float(&vla1) != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
        return 5;
    }

    if sum_pointed_ints(&vla2) != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116) {
        return 6;
    }

    if takes_params(&vla1, &vla2) != (vla1[0] as i32 + vla1[10] as i32 + *vla2[0].unwrap() as i32 + *vla2[16].unwrap() as i32) {
        return 7;
    }

    0
}