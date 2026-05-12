use std::mem::size_of;

type Usize = usize;

fn pos(calls: &mut i32, x: i32) -> i32 {
    *calls += 1;
    if x <= 0 {
        1
    } else {
        x
    }
}

fn sum_ints_from_float(p: &[f32], n: i32) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += p[i as usize] as i32;
        i += 1;
    }
    s
}

fn sum_pointed_ints(pp: &[&f32], n: i32) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += (*pp[i as usize]) as i32;
        i += 1;
    }
    s
}

fn takes_params(a: &[f32], afp2: &[&f32]) -> i32 {
    let s1 = a[0] as i32 + a[10] as i32;
    let s2 = (*afp2[0]) as i32 + (*afp2[16]) as i32;
    s1 + s2
}

fn main() {
    let mut fa: [f32; 11] = [0.0; 11];
    let mut backing: [f32; 17] = [0.0; 17];
    let mut afp: Vec<&f32> = Vec::with_capacity(17);
    for i in 0..11 {
        fa[i] = (i + 1) as f32;
    }
    for i in 0..17 {
        backing[i] = (100 + i) as f32;
        afp.push(&backing[i]);
    }

    if fa[0] != 1.0f32 || fa[10] != 11.0f32 {
        std::process::exit(1);
    }

    if (*afp[0]) as i32 != 100 || (*afp[16]) as i32 != 116 {
        std::process::exit(2);
    }

    let mut calls = 0;
    let n1 = pos(&mut calls, 11);
    let n2 = pos(&mut calls, 17);

    let mut vla1: Vec<f32> = Vec::with_capacity(n1 as usize);
    for i in 0..n1 {
        vla1.push(fa[i as usize] * 2.0f32);
    }

    let mut vla2: Vec<&f32> = Vec::with_capacity(n2 as usize);
    for i in 0..n2 {
        vla2.push(afp[i as usize]);
    }

    if calls != 2 {
        std::process::exit(3);
    }

    let start: usize = &vla1[0] as *const f32 as usize;
    let end: usize = &vla1[(n1 - 1) as usize] as *const f32 as usize;
    let span = end - start;
    let expected = ((n1 - 1) as usize) * size_of::<f32>();
    if span != expected {
        std::process::exit(4);
    }

    if sum_ints_from_float(&vla1, 11) != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
        std::process::exit(5);
    }

    if sum_pointed_ints(&vla2, 17) !=
        (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 +
         112 + 113 + 114 + 115 + 116) {
        std::process::exit(6);
    }

    if takes_params(&vla1, &vla2) !=
        (vla1[0] as i32 + vla1[10] as i32 + (*vla2[0]) as i32 + (*vla2[16]) as i32) {
        std::process::exit(7);
    }

    std::process::exit(0);
}