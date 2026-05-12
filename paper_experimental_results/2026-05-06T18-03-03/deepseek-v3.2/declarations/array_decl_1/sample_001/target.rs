fn main() {
    // Equivalent of static volatile int calls;
    static mut CALLS: i32 = 0;

    fn pos(x: i32) -> i32 {
        unsafe {
            CALLS += 1;
        }
        if x <= 0 {
            1
        } else {
            x
        }
    }

    let mut fa: [f32; 11] = [0.0; 11];
    let mut afp: [*const f32; 17] = [std::ptr::null(); 17];
    let mut backing: [f32; 17] = [0.0; 17];

    fn init_globals(fa: &mut [f32; 11], afp: &mut [*const f32; 17], backing: &mut [f32; 17]) {
        for i in 0..11 {
            fa[i] = (i + 1) as f32;
        }
        for i in 0..17 {
            backing[i] = (100 + i) as f32;
            afp[i] = &backing[i] as *const f32;
        }
    }

    fn sum_ints_from_float(p: &[f32], n: usize) -> i32 {
        let mut s = 0;
        for i in 0..n {
            s += p[i] as i32;
        }
        s
    }

    fn sum_pointed_ints(pp: &[*const f32], n: usize) -> i32 {
        let mut s = 0;
        for i in 0..n {
            unsafe {
                s += (*pp[i]) as i32;
            }
        }
        s
    }

    fn takes_params(a: &[f32; 11], afp2: &[*const f32; 17]) -> i32 {
        let s1 = a[0] as i32 + a[10] as i32;
        unsafe {
            let s2 = (*afp2[0]) as i32 + (*afp2[16]) as i32;
            s1 + s2
        }
    }

    init_globals(&mut fa, &mut afp, &mut backing);

    if fa[0] != 1.0 || fa[10] != 11.0 {
        std::process::exit(1);
    }

    unsafe {
        if (*afp[0]) as i32 != 100 || (*afp[16]) as i32 != 116 {
            std::process::exit(2);
        }
    }

    {
        let n1 = pos(11);
        let n2 = pos(17);

        let mut vla1: Vec<f32> = vec![0.0; n1 as usize];
        let mut vla2: Vec<*const f32> = vec![std::ptr::null(); n2 as usize];

        for i in 0..n1 as usize {
            vla1[i] = fa[i] * 2.0;
        }

        for i in 0..n2 as usize {
            vla2[i] = afp[i];
        }

        unsafe {
            if CALLS != 2 {
                std::process::exit(3);
            }
        }

        {
            let span = (&vla1[n1 as usize - 1] as *const f32 as usize)
                .wrapping_sub(&vla1[0] as *const f32 as usize);
            if span != ((n1 - 1) as usize) * std::mem::size_of::<f32>() {
                std::process::exit(4);
            }
        }

        if sum_ints_from_float(&vla1, 11) != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
            std::process::exit(5);
        }

        if sum_pointed_ints(&vla2, 17)
            != (100
                + 101
                + 102
                + 103
                + 104
                + 105
                + 106
                + 107
                + 108
                + 109
                + 110
                + 111
                + 112
                + 113
                + 114
                + 115
                + 116)
        {
            std::process::exit(6);
        }

        // Convert slices to fixed-size arrays for takes_params
        let vla1_arr: [f32; 11] = vla1.try_into().unwrap();
        let vla2_arr: [*const f32; 17] = vla2.try_into().unwrap();
        if takes_params(&vla1_arr, &vla2_arr)
            != (vla1[0] as i32 + vla1[10] as i32
                + unsafe { (*vla2[0]) as i32 + (*vla2[16]) as i32 })
        {
            std::process::exit(7);
        }
    }

    std::process::exit(0);
}