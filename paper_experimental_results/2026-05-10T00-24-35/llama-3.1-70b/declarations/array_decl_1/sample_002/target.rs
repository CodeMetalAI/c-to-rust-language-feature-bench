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

    let mut fa = [0.0f32; 11];
    let mut backing = [0.0f32; 17];
    let mut afp = [std::ptr::null::<f32>(); 17];

    fn init_globals() {
        for i in 0..11 {
            fa[i] = (i as f32) + 1.0;
        }
        for i in 0..17 {
            backing[i] = (100.0 + i as f32);
            afp[i] = backing[i].as_mut_ptr();
        }
    }

    fn sum_ints_from_float(p: &[f32], n: usize) -> i32 {
        p.iter().take(n).map(|x| *x as i32).sum()
    }

    fn sum_pointed_ints(pp: &[*mut f32], n: usize) -> i32 {
        pp.iter().take(n).map(|x| **x as i32).sum()
    }

    fn takes_params(a: [f32; 11], afp2: [&mut f32; 17]) -> i32 {
        (a[0] as i32) + (a[10] as i32) + (*afp2[0] as i32) + (*afp2[16] as i32)
    }

    init_globals();

    if fa[0]!= 1.0 || fa[10]!= 11.0 {
        return;
    }

    if *afp[0]!= 100.0 || *afp[16]!= 116.0 {
        return;
    }

    let n1 = pos(11);
    let n2 = pos(17);

    let mut vla1 = [0.0f32; n1 as usize];
    let mut vla2 = [std::ptr::null::<f32>(); n2 as usize];

    for i in 0..n1 as usize {
        vla1[i] = fa[i] * 2.0;
    }

    for i in 0..n2 as usize {
        vla2[i] = afp[i];
    }

    if unsafe { CALLS }!= 2 {
        return;
    }

    let span = (vla1.as_ptr() as *const u8).offset((n1 as isize) - 1) as usize -
               (vla1.as_ptr() as *const u8) as usize;
    if span!= (n1 as usize - 1) * std::mem::size_of::<f32>() {
        return;
    }

    if sum_ints_from_float(&vla1, 11)!=
       (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
        return;
    }

    if sum_pointed_ints(&vla2, 17)!=
       (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 +
        112 + 113 + 114 + 115 + 116) {
        return;
    }

    if takes_params(vla1, vla2)!=
       ((vla1[0] as i32) + (vla1[10] as i32) + (*vla2[0] as i32) + (*vla2[16] as i32)) {
        return;
    }

    println!("All tests passed");
}