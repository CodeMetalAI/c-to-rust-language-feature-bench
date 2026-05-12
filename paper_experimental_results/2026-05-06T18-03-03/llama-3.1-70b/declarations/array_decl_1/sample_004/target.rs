fn main() {
    let mut calls = 0;

    fn pos(x: i32) -> i32 {
        calls += 1;
        if x <= 0 {
            1
        } else {
            x
        }
    }

    let mut fa: [f32; 11] = [0.0; 11];
    let mut afp: [Option<&mut f32>; 17] = [None; 17];
    let mut backing: [f32; 17] = [0.0; 17];

    fn init_globals(fa: &mut [f32; 11], afp: &mut [Option<&mut f32>; 17], backing: &mut [f32; 17]) {
        for i in 0..11 {
            fa[i] = (i as f32) + 1.0;
        }
        for i in 0..17 {
            backing[i] = (100.0 + i as f32);
            afp[i] = Some(&mut backing[i]);
        }
    }

    fn sum_ints_from_float(p: &[f32], n: usize) -> i32 {
        let mut s = 0;
        for i in 0..n {
            s += p[i] as i32;
        }
        s
    }

    fn sum_pointed_ints(pp: &[Option<&mut f32>], n: usize) -> i32 {
        let mut s = 0;
        for i in 0..n {
            if let Some(p) = pp[i] {
                s += *p as i32;
            }
        }
        s
    }

    fn takes_params(a: &[f32; 11], afp2: &[Option<&mut f32>; 17]) -> i32 {
        let s1 = a[0] as i32 + a[10] as i32;
        let s2 = afp2[0].unwrap() as i32 + afp2[16].unwrap() as i32;
        s1 + s2
    }

    init_globals(&mut fa, &mut afp, &mut backing);

    if fa[0]!= 1.0 || fa[10]!= 11.0 {
        return;
    }

    if *afp[0].unwrap()!= 100.0 || *afp[16].unwrap()!= 116.0 {
        return;
    }

    let n1 = pos(11);
    let n2 = pos(17);

    let mut vla1: Vec<f32> = Vec::with_capacity(n1 as usize);
    let mut vla2: Vec<Option<&mut f32>> = Vec::with_capacity(n2 as usize);

    for i in 0..n1 {
        vla1.push(fa[i as usize] * 2.0);
    }

    for i in 0..n2 {
        vla2.push(afp[i as usize].clone());
    }

    if calls!= 2 {
        return;
    }

    let span = (vla1.as_ptr() as *const f32).offset((n1 - 1) as isize) as usize
        - (vla1.as_ptr() as *const f32) as usize;
    if span!= (n1 - 1) as usize * std::mem::size_of::<f32>() {
        return;
    }

    if sum_ints_from_float(&vla1, 11)!= (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
        return;
    }

    if sum_pointed_ints(&vla2, 17)!= (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116) {
        return;
    }

    if takes_params(&vla1, &vla2)!= ((vla1[0] as i32) + (vla1[10] as i32) + (*vla2[0].unwrap() as i32) + (*vla2[16].unwrap() as i32)) {
        return;
    }
}