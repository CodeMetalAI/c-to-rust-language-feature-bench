// array_decl_1.rs
type Usize = usize;

static mut CALLS: i32 = 0;

fn pos(x: i32) -> i32 {
    unsafe {
        CALLS += 1;
    }
    if x <= 0 {
        return 1;
    }
    x
}

fn init_globals(fa: &mut [f32; 11], afp: &mut [*const f32; 17], backing: &mut [f32; 17]) {
    for i in 0..11 {
        fa[i] = (i + 1) as f32;
    }
    for i in 0..17 {
        backing[i] = (100 + i) as f32;
        afp[i] = &backing[i];
    }
}

fn sum_ints_from_float(p: &[f32], n: usize) -> i32 {
    let mut s = 0;
    for &value in p.iter().take(n) {
        s += value as i32;
    }
    s
}

fn sum_pointed_ints(pp: &[*const f32], n: usize) -> i32 {
    let mut s = 0;
    for &ptr in pp.iter().take(n) {
        unsafe {
            s += *ptr as i32;
        }
    }
    s
}

fn takes_params(a: &[f32; 11], afp2: &[*const f32; 17]) -> i32 {
    let s1 = a[0] as i32 + a[10] as i32;
    let s2 = unsafe { *afp2[0] as i32 + *afp2[16] as i32 };
    s1 + s2
}

fn main() {
    let mut fa = [0.0f32; 11];
    let mut afp: [*const f32; 17] = [std::ptr::null(); 17];
    let mut backing = [0.0f32; 17];

    init_globals(&mut fa, &mut afp, &mut backing);

    if fa[0] != 1.0 || fa[10] != 11.0 {
        std::process::exit(1);
    }

    unsafe {
        if *afp[0] as i32 != 100 || *afp[16] as i32 != 116 {
            std::process::exit(2);
        }
    }

    let n1 = pos(11) as usize;
    let n2 = pos(17) as usize;

    let mut vla1 = vec![0.0f32; n1];
    let mut vla2 = vec![std::ptr::null(); n2];

    for i in 0..n1 {
        vla1[i] = fa[i] * 2.0;
    }

    for i in 0..n2 {
        vla2[i] = afp[i];
    }

    unsafe {
        if CALLS != 2 {
            std::process::exit(3);
        }
    }

    let span = {
        let start = &vla1[0] as *const f32 as *const u8;
        let end = &vla1[n1 - 1] as *const f32 as *const u8;
        end as Usize - start as Usize
    };

    if span != (n1 - 1) * std::mem::size_of::<f32>() {
        std::process::exit(4);
    }

    if sum_ints_from_float(&vla1, 11) != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
        std::process::exit(5);
    }

    if sum_pointed_ints(&vla2, 17) != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116) {
        std::process::exit(6);
    }

    if takes_params(&vla1.try_into().unwrap(), &vla2.try_into().unwrap()) != (vla1[0] as i32 + vla1[10] as i32 + unsafe { *vla2[0] as i32 } + unsafe { *vla2[16] as i32 }) {
        std::process::exit(7);
    }
}