fn main() {
    let mut calls = 0;

    fn pos(calls: &mut i32, x: i32) -> i32 {
        *calls += 1;
        if x <= 0 { 1 } else { x }
    }

    fn init_globals(fa: &mut [f32; 11], backing: &mut [f32; 17], afp: &mut Vec<&f32>) {
        for i in 0..11 {
            fa[i] = (i + 1) as f32;
        }
        for i in 0..17 {
            backing[i] = (100.0 + i as f32);
            afp.push(&backing[i]);
        }
    }

    fn sum_ints_from_float(p: &[f32]) -> i32 {
        p.iter().map(|&x| x as i32).sum()
    }

    fn sum_pointed_ints(pp: &[&f32]) -> i32 {
        pp.iter().map(|&&x| x as i32).sum()
    }

    fn takes_params(a: &[f32], afp2: &[&f32]) -> i32 {
        a[0] as i32 + a[10] as i32 + (*afp2[0]) as i32 + (*afp2[16]) as i32
    }

    let mut fa = [0.0f32; 11];
    let mut backing = [0.0f32; 17];
    let mut afp = Vec::new();
    init_globals(&mut fa, &mut backing, &mut afp);

    if fa[0] != 1.0f32 || fa[10] != 11.0f32 {
        std::process::exit(1);
    }

    if (*afp[0]) as i32 != 100 || (*afp[16]) as i32 != 116 {
        std::process::exit(2);
    }

    {
        let n1 = pos(&mut calls, 11);
        let n2 = pos(&mut calls, 17);
        let mut vla1 = vec![0.0f32; n1 as usize];
        let mut vla2 = Vec::with_capacity(n2 as usize);
        for i in 0..(n1 as usize) {
            vla1[i] = fa[i] * 2.0f32;
        }
        for &ptr in &afp {
            vla2.push(ptr);
        }
        if calls != 2 {
            std::process::exit(3);
        }
        if sum_ints_from_float(&vla1[..11]) != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
            std::process::exit(5);
        }
        if sum_pointed_ints(&vla2) != (100..=116).sum::<i32>() {
            std::process::exit(6);
        }
        let expected = vla1[0] as i32 + vla1[10] as i32 + (*vla2[0]) as i32 + (*vla2[16]) as i32;
        if takes_params(&vla1[..11], &vla2) != expected {
            std::process::exit(7);
        }
    }
    std::process::exit(0);
}