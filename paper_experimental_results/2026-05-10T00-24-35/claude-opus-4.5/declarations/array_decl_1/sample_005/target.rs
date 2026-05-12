use std::cell::Cell;

thread_local! {
    static CALLS: Cell<i32> = Cell::new(0);
}

fn pos(x: i32) -> i32 {
    CALLS.with(|c| c.set(c.get() + 1));
    if x <= 0 {
        return 1;
    }
    x
}

static mut FA: [f32; 11] = [0.0; 11];
static mut AFP: [usize; 17] = [0; 17];
static mut BACKING: [f32; 17] = [0.0; 17];

fn init_globals(fa: &mut [f32; 11], backing: &mut [f32; 17], afp: &mut [usize; 17]) {
    let mut i = 0;
    while i < 11 {
        fa[i] = (i + 1) as f32;
        i += 1;
    }
    i = 0;
    while i < 17 {
        backing[i] = (100 + i) as f32;
        afp[i] = i;
        i += 1;
    }
}

fn sum_ints_from_float(p: &[f32], n: i32) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += p[i as usize] as i32;
        i += 1;
    }
    s
}

fn sum_pointed_ints(pp: &[usize], backing: &[f32], n: i32) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += backing[pp[i as usize]] as i32;
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
    let mut fa = [0.0f32; 11];
    let mut backing = [0.0f32; 17];
    let mut afp = [0usize; 17];
    
    init_globals(&mut fa, &mut backing, &mut afp);

    if fa[0] != 1.0f32 || fa[10] != 11.0f32 {
        std::process::exit(1);
    }

    if backing[afp[0]] as i32 != 100 || backing[afp[16]] as i32 != 116 {
        std::process::exit(2);
    }

    {
        let n1 = pos(11) as usize;
        let n2 = pos(17) as usize;

        let mut vla1: Vec<f32> = vec![0.0; n1];
        let mut vla2: Vec<usize> = vec![0; n2];

        let mut i = 0;
        while i < n1 {
            vla1[i] = fa[i] * 2.0f32;
            i += 1;
        }

        i = 0;
        while i < n2 {
            vla2[i] = afp[i];
            i += 1;
        }

        let calls = CALLS.with(|c| c.get());
        if calls != 2 {
            std::process::exit(3);
        }

        {
            let span: usize = (n1 - 1) * std::mem::size_of::<f32>();
            if span != (n1 - 1) * std::mem::size_of::<f32>() {
                std::process::exit(4);
            }
        }

        if sum_ints_from_float(&vla1, 11) !=
            (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22)
        {
            std::process::exit(5);
        }

        if sum_pointed_ints(&vla2, &backing, 17) !=
            (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 +
             112 + 113 + 114 + 115 + 116)
        {
            std::process::exit(6);
        }

        if takes_params(&vla1, &vla2, &backing) !=
            (vla1[0] as i32 + vla1[10] as i32 + backing[vla2[0]] as i32 + backing[vla2[16]] as i32)
        {
            std::process::exit(7);
        }
    }

    std::process::exit(0);
}