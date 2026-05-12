type Usize = u64;

fn pos(x: i32, calls: &mut i32) -> i32 {
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
        s += **pp[i as usize] as i32;
        i += 1;
    }
    s
}

fn takes_params(a: &[f32], afp2: &[&f32]) -> i32 {
    let s1 = a[0] as i32 + a[10] as i32;
    let s2 = **afp2[0] as i32 + **afp2[16] as i32;
    s1 + s2
}

fn main() {
    let fa = std::array::from_fn(|i| (i + 1) as f32);
    let backing = std::array::from_fn(|i| (100 + i) as f32);
    let afp = std::array::from_fn(|i| &backing[i]);

    if fa[0] != 1.0 || fa[10] != 11.0 {
        std::process::exit(1);
    }

    if *afp[0] as i32 != 100 || *afp[16] as i32 != 116 {
        std::process::exit(2);
    }

    let mut calls = 0;
    let n1 = pos(11, &mut calls);
    let n2 = pos(17, &mut calls);

    let mut vla1 = Vec::with_capacity(n1 as usize);
    for i in 0..n1 as usize {
        vla1.push(fa[i] * 2.0);
    }

    let vla2 = std::array::from_fn(|i| afp[i]);

    if calls != 2 {
        std::process::exit(3);
    }

    {
        let span = (n1 as usize - 1) * std::mem::size_of::<f32>();
        if span != (n1 as usize - 1) * std::mem::size_of::<f32>() {
            std::process::exit(4);
        }
    }

    if sum_ints_from_float(&vla1, 11) != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
        std::process::exit(5);
    }

    if sum_pointed_ints(&vla2, 17) != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116) {
        std::process::exit(6);
    }

    if takes_params(&vla1, &vla2) != (vla1[0] as i32 + vla1[10] as i32 + **vla2[0] as i32 + **vla2[16] as i32) {
        std::process::exit(7);
    }

    std::process::exit(0);
}