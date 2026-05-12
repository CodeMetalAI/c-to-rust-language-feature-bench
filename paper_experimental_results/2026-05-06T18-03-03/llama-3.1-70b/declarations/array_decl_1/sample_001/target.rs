fn main() {
    static mut CALLS: i32 = 0;

    fn pos(x: i32) -> i32 {
        unsafe { CALLS += 1 };
        if x <= 0 {
            1
        } else {
            x
        }
    }

    let mut fa: [f32; 11] = [0.0; 11];
    let mut afp: [Option<&mut f32>; 17] = [None; 17];
    let mut backing: [f32; 17] = [0.0; 17];

    fn init_globals() {
        for i in 0..11 {
            fa[i] = (i + 1) as f32;
        }
        for i in 0..17 {
            backing[i] = (100 + i) as f32;
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

    fn takes_params(a: [f32; 11], afp2: [Option<&mut f32>; 17]) -> i32 {
        let s1 = a[0] as i32 + a[10] as i32;
        let s2 = if let Some(p) = afp2[0] { *p as i32 } else { 0 } +
                 if let Some(p) = afp2[16] { *p as i32 } else { 0 };
        s1 + s2
    }

    init_globals();

    if fa[0]!= 1.0 || fa[10]!= 11.0 {
        std::process::exit(1);
    }

    if afp[0].map_or(false, |p| *p!= 100.0) || afp[16].map_or(false, |p| *p!= 116.0) {
        std::process::exit(2);
    }

    let n1 = pos(11);
    let n2 = pos(17);

    let mut vla1: Vec<f32> = vec![0.0; n1 as usize];
    let mut vla2: Vec<Option<&mut f32>> = vec![None; n2 as usize];

    for i in 0..n1 as usize {
        vla1[i] = fa[i] * 2.0;
    }

    for i in 0..n2 as usize {
        vla2[i] = afp[i].clone();
    }

    if unsafe { CALLS }!= 2 {
        std::process::exit(3);
    }

    let span = (vla1.as_ptr() as *const f32).offset(n1 as isize - 1) as usize -
               (vla1.as_ptr() as *const f32) as usize;
    if span!= (n1 as usize - 1) * std::mem::size_of::<f32>() {
        std::process::exit(4);
    }

    if sum_ints_from_float(&vla1, 11)!=
        (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
        std::process::exit(5);
    }

    if sum_pointed_ints(&vla2, 17)!=
        (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 +
         112 + 113 + 114 + 115 + 116) {
        std::process::exit(6);
    }

    if takes_params(vla1.try_into().unwrap(), vla2.try_into().unwrap())!=
        (vla1[0] as i32 + vla1[10] as i32 + vla2[0].map_or(0, |p| *p as i32) +
         vla2[16].map_or(0, |p| *p as i32)) {
        std::process::exit(7);
    }

    std::process::exit(0);
}