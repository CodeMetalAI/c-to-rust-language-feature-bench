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

static mut FA: [f32; 11] = [0.0; 11];
static mut AFP: [*const f32; 17] = [std::ptr::null(); 17];
static mut BACKING: [f32; 17] = [0.0; 17];

fn init_globals() {
    for i in 0..11 {
        unsafe {
            FA[i] = (i + 1) as f32;
        }
    }
    for i in 0..17 {
        unsafe {
            BACKING[i] = (100 + i) as f32;
            AFP[i] = &BACKING[i];
        }
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
            s += *pp[i] as i32;
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
    init_globals();

    unsafe {
        if FA[0] != 1.0 || FA[10] != 11.0 {
            std::process::exit(1);
        }

        if *AFP[0] as i32 != 100 || *AFP[16] as i32 != 116 {
            std::process::exit(2);
        }
    }

    let n1 = pos(11) as usize;
    let n2 = pos(17) as usize;

    let mut vla1 = vec![0.0f32; n1];
    let mut vla2 = vec![std::ptr::null(); n2];

    for i in 0..n1 {
        unsafe {
            vla1[i] = FA[i] * 2.0;
        }
    }

    for i in 0..n2 {
        unsafe {
            vla2[i] = AFP[i];
        }
    }

    unsafe {
        if CALLS != 2 {
            std::process::exit(3);
        }
    }

    {
        let span = (vla1.as_ptr().wrapping_offset((n1 - 1) as isize) as usize)
            - (vla1.as_ptr() as usize);
        if span != (n1 - 1) * std::mem::size_of::<f32>() {
            std::process::exit(4);
        }
    }

    if sum_ints_from_float(&vla1, 11)
        != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22)
    {
        std::process::exit(5);
    }

    if sum_pointed_ints(&vla2, 17)
        != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111
            + 112 + 113 + 114 + 115 + 116)
    {
        std::process::exit(6);
    }

    if takes_params(&vla1.try_into().unwrap(), &vla2.try_into().unwrap())
        != (vla1[0] as i32 + vla1[10] as i32
            + unsafe { *vla2[0] as i32 + *vla2[16] as i32 })
    {
        std::process::exit(7);
    }
}