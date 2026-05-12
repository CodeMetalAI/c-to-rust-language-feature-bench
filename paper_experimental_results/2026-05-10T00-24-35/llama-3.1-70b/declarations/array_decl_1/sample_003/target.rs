fn main() {
    let mut calls: i32 = 0;

    fn pos(x: i32) -> i32 {
        unsafe { calls += 1; }
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
        for (i, fa_i) in fa.iter_mut().enumerate() {
            *fa_i = (i as f32) + 1.0;
        }
        for (i, (afp_i, backing_i)) in afp.iter_mut().zip(backing.iter_mut()).enumerate() {
            *backing_i = (100.0 + i as f32);
            *afp_i = Some(backing_i);
        }
    }

    init_globals(&mut fa, &mut afp, &mut backing);

    if fa[0]!= 1.0 || fa[10]!= 11.0 {
        return;
    }

    if afp[0].unwrap().read()!= 100.0 || afp[16].unwrap().read()!= 116.0 {
        return;
    }

    let n1 = pos(11);
    let n2 = pos(17);

    let mut vla1: Vec<f32> = vec![0.0; n1 as usize];
    let mut vla2: Vec<Option<&mut f32>> = vec![None; n2 as usize];

    for (i, (vla1_i, fa_i)) in vla1.iter_mut().zip(fa.iter()).enumerate() {
        *vla1_i = *fa_i * 2.0;
    }

    for (i, (vla2_i, afp_i)) in vla2.iter_mut().zip(afp.iter()).enumerate() {
        *vla2_i = *afp_i;
    }

    if calls!= 2 {
        return;
    }

    let span = (vla1.as_ptr() as usize) + (vla1.len() - 1) * std::mem::size_of::<f32>() - vla1.as_ptr() as usize;
    if span!= (vla1.len() - 1) * std::mem::size_of::<f32>() {
        return;
    }

    if vla1.iter().map(|x| x as i32).sum::<i32>()!= 2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22 {
        return;
    }

    if vla2.iter().map(|x| x.unwrap().read() as i32).sum::<i32>()!= 100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116 {
        return;
    }

    let takes_params = |a: &[f32], afp2: &[Option<&mut f32>]| -> i32 {
        (a[0] as i32) + (a[10] as i32) + (afp2[0].unwrap().read() as i32) + (afp2[16].unwrap().read() as i32)
    };

    if takes_params(&vla1, &vla2)!= ((vla1[0] as i32) + (vla1[10] as i32) + (vla2[0].unwrap().read() as i32) + (vla2[16].unwrap().read() as i32)) {
        return;
    }
}