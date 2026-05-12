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

static mut FA: [f32; 11] = [0.0; 11];
static mut AFP: [*mut f32; 17] = [std::ptr::null_mut(); 17];
static mut BACKING: [f32; 17] = [0.0; 17];

fn init_globals() {
    unsafe {
        let mut i = 0;
        while i < 11 {
            FA[i] = (i + 1) as f32;
            i += 1;
        }
        i = 0;
        while i < 17 {
            BACKING[i] = (100 + i) as f32;
            AFP[i] = &mut BACKING[i] as *mut f32;
            i += 1;
        }
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

fn sum_pointed_ints(pp: &[*mut f32], n: i32) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        unsafe {
            s += (*pp[i as usize]) as i32;
        }
        i += 1;
    }
    s
}

fn takes_params(a: &[f32], afp2: &[*mut f32]) -> i32 {
    unsafe {
        let s1 = a[0] as i32 + a[10] as i32;
        let s2 = (*afp2[0]) as i32 + (*afp2[16]) as i32;
        s1 + s2
    }
}

fn main() {
    init_globals();

    unsafe {
        if FA[0] != 1.0f32 || FA[10] != 11.0f32 {
            std::process::exit(1);
        }

        if (*AFP[0]) as i32 != 100 || (*AFP[16]) as i32 != 116 {
            std::process::exit(2);
        }

        {
            let n1 = pos(11);
            let n2 = pos(17);

            let mut vla1 = vec![0.0f32; n1 as usize];
            let mut vla2 = vec![std::ptr::null_mut::<f32>(); n2 as usize];

            let mut i = 0;
            while i < n1 {
                vla1[i as usize] = FA[i as usize] * 2.0f32;
                i += 1;
            }

            i = 0;
            while i < n2 {
                vla2[i as usize] = AFP[i as usize];
                i += 1;
            }

            if CALLS != 2 {
                std::process::exit(3);
            }

            {
                let span = (&vla1[(n1 - 1) as usize] as *const f32 as usize)
                    - (&vla1[0] as *const f32 as usize);
                if span != ((n1 - 1) as Usize) * (std::mem::size_of::<f32>() as Usize) {
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

            if takes_params(&vla1, &vla2)
                != (vla1[0] as i32 + vla1[10] as i32 + (*vla2[0]) as i32 + (*vla2[16]) as i32)
            {
                std::process::exit(7);
            }
        }
    }

    std::process::exit(0);
}