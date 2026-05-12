fn main() {
    let mut calls: i32 = 0;

    fn pos(x: i32) -> i32 {
        unsafe { calls += 1 };
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
        for (i, fa) in fa.iter_mut().enumerate() {
            *fa = (i + 1) as f32;
        }
        for (i, (afp, backing)) in afp.iter_mut().zip(backing.iter_mut()).enumerate() {
            *backing = (100.0 + i as f32);
            *afp = Some(backing);
        }
    }

    init_globals(&mut fa, &mut afp, &mut backing);

    if fa[0]!= 1.0 || fa[10]!= 11.0 {
        std::process::exit(1);
    }

    if afp[0].unwrap().read()!= 100.0 || afp[16].unwrap().read()!= 116.0 {
        std::process::exit(2);
    }

    {
        let n1 = pos(11);
        let n2 = pos(17);

        let mut vla1: Vec<f32> = vec![0.0; n1];
        let mut vla2: Vec<Option<&mut f32>> = vec![None; n2];

        for (i, vla1) in vla1.iter_mut().enumerate() {
            *vla1 = fa[i] * 2.0;
        }

        for (i, vla2) in vla2.iter_mut().enumerate() {
            *vla2 = afp[i];
        }

        if unsafe { calls }!= 2 {
            std::process::exit(3);
        }

        let span = (vla1.as_ptr() as *const u8).offset((n1 - 1) as isize) as usize
            - (vla1.as_ptr() as *const u8) as usize;
        if span!= (n1 - 1) * std::mem::size_of::<f32>() {
            std::process::exit(4);
        }

        if sum_ints_from_float(&vla1, 11)
           != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22)
        {
            std::process::exit(5);
        }

        if sum_pointed_ints(&vla2, 17)
           != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116)
        {
            std::process::exit(6);
        }

        if takes_params(&vla1, &vla2)
           != ((vla1[0] as i32) + (vla1[10] as i32) + (vla2[0].unwrap().read() as i32) + (vla2[16].unwrap().read() as i32))
        {
            std::process::exit(7);
        }
    }

    std::process::exit(0);
}

fn sum_ints_from_float(p: &[f32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += p[i] as i32;
    }
    s
}

fn sum_pointed_ints(pp: &Vec<Option<&mut f32>>, n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += pp[i].unwrap().read() as i32;
    }
    s
}

fn takes_params(a: &[f32], afp2: &Vec<Option<&mut f32>>) -> i32 {
    (a[0] as i32) + (a[10] as i32) + (afp2[0].unwrap().read() as i32) + (afp2[16].unwrap().read() as i32)
}