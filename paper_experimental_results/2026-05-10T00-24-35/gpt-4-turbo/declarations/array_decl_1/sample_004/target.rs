fn main() {
    static mut CALLS: i32 = 0;

    let mut fa = [0.0f32; 11];
    let mut backing = [0.0f32; 17];
    let mut afp: [*mut f32; 17] = [&mut backing[0]; 17];

    // Initialize fa and backing
    for i in 0..11 {
        fa[i] = (i + 1) as f32;
    }

    for i in 0..17 {
        backing[i] = (100 + i) as f32;
        afp[i] = &mut backing[i];
    }

    if fa[0] != 1.0 || fa[10] != 11.0 {
        std::process::exit(1);
    }
    
    if unsafe { *afp[0] as i32 } != 100 || unsafe { *afp[16] as i32 } != 116 {
        std::process::exit(2);
    }
    
    let n1 = pos(11, &mut CALLS);
    let n2 = pos(17, &mut CALLS);

    let mut vla1 = vec![0.0f32; n1];
    let mut vla2 = vec![&mut backing[0] as *mut f32; n2];

    for i in 0..n1 {
        vla1[i] = fa[i] * 2.0;
    }

    for i in 0..n2 {
        vla2[i] = afp[i];
    }

    if unsafe { CALLS } != 2 {
        std::process::exit(3);
    }

    if sum_ints_from_float(&vla1, 11) != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
        std::process::exit(5);
    }

    if sum_pointed_ints(&vla2, 17) != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116) {
        std::process::exit(6);
    }

    if takes_params(&vla1, &vla2) != (vla1[0] as i32 + vla1[10] as i32 + unsafe { *vla2[0] } as i32 + unsafe { *vla2[16] } as i32) {
        std::process::exit(7);
    }
}

fn pos(x: i32, calls: &mut i32) -> usize {
    *calls += 1;
    if x <= 0 {
        1
    } else {
        x as usize
    }
}

fn sum_ints_from_float(p: &[f32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += p[i] as i32;
    }
    s
}

fn sum_pointed_ints(pp: &[*mut f32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        unsafe {
            s += *pp[i] as i32;
        }
    }
    s
}

fn takes_params(a: &[f32], afp2: &[*mut f32]) -> i32 {
    let s1 = a[0] as i32 + a[10] as i32;
    let s2 = unsafe { *afp2[0] as i32 } + unsafe { *afp2[16] as i32 };
    s1 + s2
}