use std::mem;

static mut CALLS: i32 = 0;

fn pos(x: i32) -> i32 {
    unsafe { CALLS += 1; }
    if x <= 0 {
        1
    } else {
        x
    }
}

static FA: [f32; 11] = [0.0; 11];
static mut AFP: [*const f32; 17] = [std::ptr::null(); 17];
static BACKING: [f32; 17] = [0.0; 17];

fn init_globals() {
    let mut i = 0;
    while i < 11 {
        unsafe { FA[i] = (i as f32) + 1.0; }
        i += 1;
    }
    i = 0;
    while i < 17 {
        unsafe {
            BACKING[i] = (100 + i) as f32;
            AFP[i] = &BACKING[i];
        }
        i += 1;
    }
}

fn sum_ints_from_float(p: *const f32, n: i32) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        unsafe { s += p.add(i as usize) as i32; }
        i += 1;
    }
    s
}

fn sum_pointed_ints(pp: *const *const f32, n: i32) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        unsafe { s += (*pp.add(i as usize)) as i32; }
        i += 1;
    }
    s
}

fn takes_params(a: &[f32; 11], afp2: &[*const f32; 17]) -> i32 {
    let s1 = a[0] as i32 + a[10] as i32;
    let s2 = unsafe { (*afp2[0] as *const f32) as i32 } + unsafe { (*afp2[16] as *const f32) as i32 };
    s1 + s2
}

fn main() -> i32 {
    unsafe {
        init_globals();

        if FA[0] != 1.0 || FA[10] != 11.0 {
            return 1;
        }

        if (*AFP[0]) as i32 != 100 || (*AFP[16]) as i32 != 116 {
            return 2;
        }

        {
            let n1 = pos(11);
            let n2 = pos(17);

            let mut vla1: Vec<f32> = vec![0.0; n1];
            let mut vla2: Vec<*const f32> = vec![std::ptr::null(); n2];

            let mut i = 0;
            while i < n1 {
                vla1[i] = FA[i] * 2.0;
                i += 1;
            }

            i = 0;
            while i < n2 {
                vla2[i] = AFP[i];
                i += 1;
            }

            if CALLS != 2 {
                return 3;
            }

            {
                let span = unsafe {
                    (vla1.as_ptr() as *const u8)
                        .add(n1 - 1)
                        .offset_from(vla1.as_ptr() as *const u8) as usize
                };
                if span != (n1 - 1) * mem::size_of::<f32>() as usize {
                    return 4;
                }
            }

            if sum_ints_from_float(vla1.as_ptr(), n1 as i32)
                != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22)
            {
                return 5;
            }

            if sum_pointed_ints(vla2.as_ptr(), n2 as i32)
                != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 +
                     111 + 112 + 113 + 114 + 115 + 116)
            {
                return 6;
            }

            if takes_params(&vla1, &vla2)
                != (vla1[0] as i32 + vla1[10] as i32 + unsafe { (*vla2[0]) as i32 } +
                    unsafe { (*vla2[16]) as i32 })
            {
                return 7;
            }
        }

        0
    }
}