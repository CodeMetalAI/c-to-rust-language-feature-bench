use std::mem::size_of;
use std::process::exit;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::OnceLock;

struct Globals {
    fa: [f32; 11],
    backing: [f32; 17],
    afp: [usize; 17],
}

static CALLS: AtomicI32 = AtomicI32::new(0);
static GLOBALS: OnceLock<Globals> = OnceLock::new();

fn pos(x: i32) -> i32 {
    CALLS.fetch_add(1, Ordering::SeqCst);
    if x <= 0 {
        1
    } else {
        x
    }
}

fn init_globals() -> Globals {
    let mut fa = [0f32; 11];
    let mut i = 0;
    while i < 11 {
        fa[i] = (i + 1) as f32;
        i += 1;
    }

    let mut backing = [0f32; 17];
    let mut afp = [0usize; 17];
    i = 0;
    while i < 17 {
        backing[i] = (100 + i) as f32;
        afp[i] = i;
        i += 1;
    }

    Globals { fa, backing, afp }
}

fn sum_ints_from_float(p: &[f32], n: usize) -> i32 {
    let mut s = 0i32;
    let mut i = 0;
    while i < n {
        s += p[i] as i32;
        i += 1;
    }
    s
}

fn sum_pointed_ints(pp: &[usize], n: usize, backing: &[f32; 17]) -> i32 {
    let mut s = 0i32;
    let mut i = 0;
    while i < n {
        s += backing[pp[i]] as i32;
        i += 1;
    }
    s
}

fn takes_params(a: &[f32], afp2: &[usize], backing: &[f32; 17]) -> i32 {
    let s1 = a[0] as i32 + a[10] as i32;
    let s2 = backing[afp2[0]] as i32 + backing[afp2[16]] as i32;
    s1 + s2
}

fn main() {
    let globals = init_globals();
    GLOBALS.set(globals).unwrap();
    let g = GLOBALS.get().unwrap();

    if g.fa[0] != 1.0f32 || g.fa[10] != 11.0f32 {
        exit(1);
    }

    if (g.backing[g.afp[0]] as i32) != 100 || (g.backing[g.afp[16]] as i32) != 116 {
        exit(2);
    }

    {
        let n1 = pos(11) as usize;
        let n2 = pos(17) as usize;

        let mut vla1 = vec![0f32; n1];
        let mut vla2 = vec![0usize; n2];

        let mut i = 0;
        while i < n1 {
            vla1[i] = g.fa[i] * 2.0f32;
            i += 1;
        }

        i = 0;
        while i < n2 {
            vla2[i] = g.afp[i];
            i += 1;
        }

        if CALLS.load(Ordering::SeqCst) != 2 {
            exit(3);
        }

        {
            let span = (n1 - 1) * size_of::<f32>();
            if span != (n1 - 1) * size_of::<f32>() {
                exit(4);
            }
        }

        if sum_ints_from_float(&vla1, 11)
            != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22)
        {
            exit(5);
        }

        if sum_pointed_ints(&vla2, 17, &g.backing)
            != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111
                + 112 + 113 + 114 + 115 + 116)
        {
            exit(6);
        }

        if takes_params(&vla1, &vla2, &g.backing)
            != (vla1[0] as i32 + vla1[10] as i32 + g.backing[vla2[0]] as i32
                + g.backing[vla2[16]] as i32)
        {
            exit(7);
        }
    }

    exit(0);
}