use std::mem::size_of;
use std::process::exit;
use std::sync::atomic::{AtomicI32, Ordering};

fn pos(x: i32, calls: &AtomicI32) -> i32 {
    calls.fetch_add(1, Ordering::SeqCst);
    if x <= 0 {
        1
    } else {
        x
    }
}

fn init_globals() -> (Vec<f32>, Vec<f32>, Vec<usize>) {
    let mut fa = vec![0.0f32; 11];
    for i in 0..11 {
        fa[i] = (i as f32) + 1.0;
    }

    let mut backing = vec![0.0f32; 17];
    let mut afp = vec![0usize; 17];
    for i in 0..17 {
        backing[i] = 100.0 + (i as f32);
        afp[i] = i;
    }

    (fa, backing, afp)
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

fn sum_pointed_ints(pp: &[usize], n: usize, backing: &[f32]) -> i32 {
    let mut s = 0i32;
    let mut i = 0usize;
    while i < n {
        s += backing[pp[i]] as i32;
        i += 1;
    }
    s
}

fn takes_params(a: &[f32], afp2: &[usize], backing: &[f32]) -> i32 {
    let s1 = a[0] as i32 + a[10] as i32;
    let s2 = backing[afp2[0]] as i32 + backing[afp2[16]] as i32;
    s1 + s2
}

fn main() {
    let calls = AtomicI32::new(0);

    let (fa, backing, afp) = init_globals();

    if fa[0] != 1.0f32 || fa[10] != 11.0f32 {
        exit(1);
    }

    if backing[afp[0]] as i32 != 100 || backing[afp[16]] as i32 != 116 {
        exit(2);
    }

    {
        let n1 = pos(11, &calls) as usize;
        let n2 = pos(17, &calls) as usize;

        let mut vla1 = vec![0.0f32; n1];
        let mut vla2 = vec![0usize; n2];

        let mut i = 0usize;
        while i < n1 {
            vla1[i] = fa[i] * 2.0f32;
            i += 1;
        }

        i = 0;
        while i < n2 {
            vla2[i] = afp[i];
            i += 1;
        }

        if calls.load(Ordering::SeqCst) != 2 {
            exit(3);
        }

        {
            let span: usize = (n1 - 1) * size_of::<f32>();
            if span != (n1 - 1) * size_of::<f32>() {
                exit(4);
            }
        }

        if sum_ints_from_float(&vla1, 11)
            != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22)
        {
            exit(5);
        }

        if sum_pointed_ints(&vla2, 17, &backing)
            != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111
                + 112 + 113 + 114 + 115 + 116)
        {
            exit(6);
        }

        if takes_params(&vla1, &vla2, &backing)
            != (vla1[0] as i32
                + vla1[10] as i32
                + backing[vla2[0]] as i32
                + backing[vla2[16]] as i32)
        {
            exit(7);
        }
    }

    exit(0);
}