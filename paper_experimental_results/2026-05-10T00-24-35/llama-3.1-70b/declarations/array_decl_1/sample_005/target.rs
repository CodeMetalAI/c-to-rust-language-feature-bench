fn main() {
    static mut calls: usize = 0;

    fn pos(x: usize) -> usize {
        unsafe { calls += 1; }
        if x <= 0 {
            1
        } else {
            x
        }
    }

    let mut fa: [f32; 11] = [0.0; 11];
    let mut afp: [&mut f32; 17] = [(); 17].map(|_| &mut 0.0);
    let mut backing: [f32; 17] = [0.0; 17];

    fn init_globals(fa: &mut [f32; 11], afp: &mut [&mut f32; 17], backing: &mut [f32; 17]) {
        for (i, elem) in fa.iter_mut().enumerate() {
            *elem = (i + 1) as f32;
        }
        for (i, elem) in backing.iter_mut().enumerate() {
            *elem = (100 + i) as f32;
            afp[i] = elem;
        }
    }

    fn sum_ints_from_float(p: &[f32], n: usize) -> i32 {
        p.iter().take(n).map(|x| *x as i32).sum()
    }

    fn sum_pointed_ints(pp: &[&f32], n: usize) -> i32 {
        pp.iter().take(n).map(|x| **x as i32).sum()
    }

    fn takes_params(a: &[f32; 11], afp2: &[&f32; 17]) -> i32 {
        (a[0] + a[10]) as i32 + (**afp2[0] + **afp2[16]) as i32
    }

    init_globals(&mut fa, &mut afp, &mut backing);

    if fa[0] != 1.0 || fa[10] != 11.0 {
        std::process::exit(1);
    }

    if *afp[0] != 100.0 || *afp[16] != 116.0 {
        std::process::exit(2);
    }

    {
        let n1 = pos(11);
        let n2 = pos(17);

        let mut vla1: [f32; 11] = [0.0; 11];
        let mut vla2: [&mut f32; 17] = [(); 17].map(|_| &mut 0.0);

        for (i, elem) in vla1.iter_mut().enumerate() {
            *elem = fa[i] * 2.0;
        }

        for (i, elem) in vla2.iter_mut().enumerate() {
            *elem = afp[i];
        }

        if unsafe { calls } != 2 {
            std::process::exit(3);
        }

        {
            let span = (vla1.as_ptr() as usize).wrapping_sub(vla1.as_ptr() as usize) as usize;
            if span != (n1 - 1) * std::mem::size_of::<f32>() {
                std::process::exit(4);
            }
        }

        if sum_ints_from_float(&vla1, 11) != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
            std::process::exit(5);
        }

        if sum_pointed_ints(&vla2, 17) != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116) {
            std::process::exit(6);
        }

        if takes_params(&vla1, &vla2) != ((vla1[0] as i32) + (vla1[10] as i32) + (*vla2[0] as i32) + (*vla2[16] as i32)) {
            std::process::exit(7);
        }
    }

    std::process::exit(0);
}