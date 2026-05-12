use std::sync::atomic::{AtomicI32, Ordering};
use std::mem;

static CALLS: AtomicI32 = AtomicI32::new(0);

fn pos(x: i32) -> i32 {
    CALLS.fetch_add(1, Ordering::Relaxed);
    if x <= 0 {
        1
    } else {
        x
    }
}

fn init_globals(fa: &mut [f32; 11], backing: &mut [f32; 17], afp: &mut [Option<&f32>; 17]) {
    for i in 0..11 {
        fa[i] = (i + 1) as f32;
    }
    for i in 0..17 {
        backing[i] = (100 + i) as f32;
        afp[i] = Some(&backing[i]);
    }
}

fn sum_ints_from_float(p: &[f32], n: i32) -> i32 {
    let mut s = 0;
    for i in 0..(n as usize) {
        s += p[i] as i32;
    }
    s
}

fn sum_pointed_ints(pp: &[&f32], n: i32) -> i32 {
    let mut s = 0;
    for i in 0..(n as usize) {
        s += *pp[i] as i32;
    }
    s
}

fn takes_params(a: &[f32], afp2: &[&f32]) -> i32 {
    let s1 = a[0] as i32 + a[10] as i32;
    let s2 = *afp2[0] as i32 + *afp2[16] as i32;
    s1 + s2
}

fn main() {
    let mut fa = [0.0f32; 11];
    let mut backing = [0.0f32; 17];
    let mut afp = [None::<&f32>; 17];
    init_globals(&mut fa, &mut backing, &mut afp);

    if fa[0] != 1.0f32 || fa[10] != 11.0f32 {
        std::process::exit(1);
    }

    if (*afp[0].unwrap() as i32) != 100 || (*afp[16].unwrap() as i32) != 116 {
        std::process::exit(2);
    }

    {
        let n1 = pos(11);
        let n2 = pos(17);

        let mut vla1 = [0.0f32; 11];
        let mut vla2 = [None::<&f32>; 17];

        for i in 0..(n1 as usize) {
            vla1[i] = fa[i] * 2.0f32;
        }

        for i in 0..(n2 as usize) {
            vla2[i] = afp[i];
        }

        if CALLS.load(Ordering::Relaxed) != 2 {
            std::process::exit(3);
        }

        {
            let span = mem::size_of_val(&vla1) - mem::size_of::<f32>();
            if span as u64 != ((n1 as u64) - 1) * (mem::size_of::<f32>() as u64) {
                std::process::exit(4);
            }
        }

        if sum_ints_from_float(&vla1[..], 11) != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
            std::process::exit(5);
        }

        let vla2_refs: Vec<&f32> = vla2.iter().map(|x| x.unwrap()).collect();
        if sum_pointed_ints(&vla2_refs, 17) != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116) {
            std::process::exit(6);
        }

        if takes_params(&vla1[..], &vla2_refs) != ((vla1[0] as i32) + (vla1[10] as i32) + (*vla2_refs[0] as i32) + (*vla2_refs[16] as i32)) {
            std::process::exit(7);
        }
    }

    std::process::exit(0);
}