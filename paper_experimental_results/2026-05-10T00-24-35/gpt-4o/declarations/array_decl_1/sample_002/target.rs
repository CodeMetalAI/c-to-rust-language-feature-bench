// array_decl_1.rs
type Usize = usize;

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

fn init_globals() -> ([f32; 11], [f32; 17], [&f32; 17]) {
    let mut fa = [0.0f32; 11];
    let mut afp = [&fa[0]; 17];
    let mut backing = [0.0f32; 17];

    for i in 0..11 {
        fa[i] = (i + 1) as f32;
    }

    for i in 0..17 {
        backing[i] = (100 + i) as f32;
        afp[i] = &backing[i];
    }

    (fa, backing, afp)
}

fn sum_ints_from_float(p: &[f32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += p[i] as i32;
    }
    s
}

fn sum_pointed_ints(pp: &[&f32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += *pp[i] as i32;
    }
    s
}

fn takes_params(a: &[f32; 11], afp2: &[&f32; 17]) -> i32 {
    let s1 = a[0] as i32 + a[10] as i32;
    let s2 = *afp2[0] as i32 + *afp2[16] as i32;
    s1 + s2
}

fn main() {
    let (fa, backing, afp) = init_globals();

    if fa[0] != 1.0 || fa[10] != 11.0 {
        std::process::exit(1);
    }

    if *afp[0] as i32 != 100 || *afp[16] as i32 != 116 {
        std::process::exit(2);
    }

    let n1 = pos(11);
    let n2 = pos(17);

    let mut vla1 = vec![0.0f32; n1 as usize];
    let mut vla2 = vec![&backing[0]; n2 as usize];

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
            - (&vla1[0] as *const f32 as usize);
        if span != (n1 as usize - 1) * std::mem::size_of::<f32>() {
            std::process::exit(4);
        }
    }

    if sum_ints_from_float(&vla1, 11) != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
        std::process::exit(5);
    }

    if sum_pointed_ints(&vla2, 17) != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116) {
        std::process::exit(6);
    }

    if takes_params(&vla1.try_into().unwrap(), &vla2.try_into().unwrap()) != ((vla1[0] as i32) + (vla1[10] as i32) + (*vla2[0] as i32) + (*vla2[16] as i32)) {
        std::process::exit(7);
    }

    std::process::exit(0);
}