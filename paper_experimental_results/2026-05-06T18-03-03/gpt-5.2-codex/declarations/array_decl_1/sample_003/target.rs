use std::sync::atomic::{AtomicI32, Ordering};

static CALLS: AtomicI32 = AtomicI32::new(0);

fn pos(x: i32) -> i32 {
    CALLS.fetch_add(1, Ordering::SeqCst);
    if x <= 0 {
        1
    } else {
        x
    }
}

fn init_globals<'a>(fa: &mut [f32; 11], backing: &'a mut [f32; 17]) -> Vec<&'a f32> {
    let mut i = 0;
    while i < 11 {
        fa[i] = (i as f32) + 1.0;
        i += 1;
    }
    i = 0;
    while i < 17 {
        backing[i] = 100.0 + (i as f32);
        i += 1;
    }
    let mut afp: Vec<&f32> = Vec::with_capacity(17);
    for i in 0..17 {
        afp.push(&backing[i]);
    }
    afp
}

fn sum_ints_from_float(p: &[f32], n: usize) -> i32 {
    let mut s = 0i32;
    let mut i = 0usize;
    while i < n {
        s += p[i] as i32;
        i += 1;
    }
    s
}

fn sum_pointed_ints(pp: &[&f32], n: usize) -> i32 {
    let mut s = 0i32;
    let mut i = 0usize;
    while i < n {
        s += *pp[i] as i32;
        i += 1;
    }
    s
}

fn takes_params(a: &[f32], afp2: &[&f32]) -> i32 {
    let s1 = a[0] as i32 + a[10] as i32;
    let s2 = *afp2[0] as i32 + *afp2[16] as i32;
    s1 + s2
}

fn run() -> i32 {
    let mut fa = [0.0f32; 11];
    let mut backing = [0.0f32; 17];
    let afp = init_globals(&mut fa, &mut backing);

    if fa[0] != 1.0f32 || fa[10] != 11.0f32 {
        return 1;
    }

    if *afp[0] as i32 != 100 || *afp[16] as i32 != 116 {
        return 2;
    }

    {
        let n1 = pos(11) as usize;
        let n2 = pos(17) as usize;

        let mut vla1 = vec![0.0f32; n1];
        let mut vla2: Vec<&f32> = Vec::with_capacity(n2);

        let mut i = 0usize;
        while i < n1 {
            vla1[i] = fa[i] * 2.0f32;
            i += 1;
        }

        i = 0;
        while i < n2 {
            vla2.push(afp[i]);
            i += 1;
        }

        if CALLS.load(Ordering::SeqCst) != 2 {
            return 3;
        }

        {
            let span: usize = (n1 - 1) * std::mem::size_of::<f32>();
            if span != (n1 - 1) * std::mem::size_of::<f32>() {
                return 4;
            }
        }

        if sum_ints_from_float(&vla1, 11)
            != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22)
        {
            return 5;
        }

        if sum_pointed_ints(&vla2, 17)
            != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110
                + 111
                + 112
                + 113
                + 114
                + 115
                + 116)
        {
            return 6;
        }

        if takes_params(&vla1, &vla2)
            != (vla1[0] as i32 + vla1[10] as i32 + *vla2[0] as i32 + *vla2[16] as i32)
        {
            return 7;
        }
    }

    0
}

fn main() {
    std::process::exit(run());
}