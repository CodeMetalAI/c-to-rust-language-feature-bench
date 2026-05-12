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
    let mut calls: i32 = 0;

    let mut fa: [f32; 11] = [0.0; 11];
    let mut backing: [f32; 17] = [0.0; 17];
    let mut afp: [&f32; 17] = [&0.0; 17];

    // init_globals
    let mut i = 0;
    while i < 11 {
        fa[i] = (i as f32) + 1.0;
        i += 1;
    }
    i = 0;
    while i < 17 {
        backing[i] = 100.0 + (i as f32);
        afp[i] = &backing[i];
        i += 1;
    }

    if fa[0] != 1.0 || fa[10] != 11.0 {
        std::process::exit(1);
    }

    if (*afp[0] as i32) != 100 || (*afp[16] as i32) != 116 {
        std::process::exit(2);
    }

    {
        let n1 = pos(11, &mut calls);
        let n2 = pos(17, &mut calls);

        let mut vla1: Vec<f32> = vec![0.0; n1 as usize];
        let mut vla2: Vec<&f32> = vec![&0.0; n2 as usize];

        i = 0;
        while (i as usize) < n1 as usize {
            vla1[i as usize] = fa[i as usize] * 2.0;
            i += 1;
        }

        i = 0;
        while (i as usize) < n2 as usize {
            vla2[i as usize] = afp[i as usize];
            i += 1;
        }

        if calls != 2 {
            std::process::exit(3);
        }

        // Span check: In Rust, Vec is guaranteed contiguous, so this always holds.
        // No need for explicit check as it preserves behavior.

        if sum_ints_from_float(&vla1, 11) != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
            std::process::exit(5);
        }

        if sum_pointed_ints(&vla2, 17) != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116) {
            std::process::exit(6);
        }

        if takes_params(&vla1, &vla2) != ((vla1[0] as i32) + (vla1[10] as i32) + ((*vla2[0]) as i32) + ((*vla2[16]) as i32)) {
            std::process::exit(7);
        }
    }

    std::process::exit(0);
}