fn main() {
    static mut CALLS: i32 = 0;

    fn pos(x: i32) -> i32 {
        CALLS += 1;
        if x <= 0 {
            1
        } else {
            x
        }
    }

    let mut fa = [0.0f32; 11];
    let mut afp: [*mut f32; 17] = [std::ptr::null_mut(); 17];
    let mut backing = [0.0f32; 17];

    fn init_globals(fa: &mut [f32; 11], afp: &mut [*mut f32; 17], backing: &mut [f32; 17]) {
        for i in 0..11 {
            fa[i] = (i + 1) as f32;
        }
        for i in 0..17 {
            backing[i] = (100 + i) as f32;
            afp[i] = &mut backing[i];
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
        for i in -..n {
            s += *pp[i] as i32;
        }
        s
    }

    fn takes_params(a: &[f32], afp2: &[*mut f32]) -> i32 {
        let s1 = a[0] as i32 + a[10] as i32;
        let s2 = *afp2[0] as i32 + *afp2[16] as i32;
        s1 + s2
    }

    init_globals(&mut fa, &mut afp, &mut backing);

    if fa[0] != 1.0f32 || fa[10] != 11.0f32 {
        return 1;
    }

    if *afp[0] as i32 != 100 || *afp[16] as i32 != 116 {
        return 2;
    }

    let n1 = pos(11);
    let n2 = pos(17);

    let mut vla1: Vec<f32> = Vec::with_capacity(n1 as usize);
    for i in 0..n1 as usize {
        vla1.push(fa[i] * 2.0f32);
    }

    let mut vla2: Vec<*mut f32> = Vec::with_capacity(n2 as usize);
    for i in 0..n2 as usize {
        vla2.push(afp[i]);
    }

    if CALLS != 2 {
        return 3;
    }

    {
        let span = (vla1.len() - 1) * std::mem::size_of::<f32>();
        if span != (n1 as usize - 1) * std::mem::size_of::<f32>() {
            return 4;
        }
    }

    if sum_ints_from_float(&vla1, 11) != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
        return 5;
    }

    if sum_pointed_ints(&vla2, 17) != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116) {
        return 6;
    }

    if takes_params(&vla1, &vla2) != (vla1[0] as i32 + vla1[10] as i32 + *vla2[0] as i32 + *vla2[16] as i32) {
        return 7;
    }

    0
}