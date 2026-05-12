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
static mut AFP: [Option<&f32>; 17] = [None; 17];
static BACKING: [f32; 17] = [0.0; 17];

static fn init_globals() {
    let mut i = 0;
    while i < 11 {
        unsafe { FA[i] = (i + 1) as f32; }
        i += 1;
    }
    i = 0;
    while i < 17 {
        unsafe {
            BACKING[i] = (100 + i) as f32;
            AFP[i] = Some(&BACKING[i]);
        }
        i += 1;
    }
}

static fn sum_ints_from_float(p: &[f32], n: i32) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += p[i as usize] as i32;
        i += 1;
    }
    s
}

static fn sum_pointed_ints(pp: &[Option<&f32>], n: i32) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        if let Some(ptr) = pp[i as usize] {
            s += *ptr as i32;
        }
        i += 1;
    }
    s
}

fn takes_params(a: &[f32; 11], afp2: &[Option<&f32>; 17]) -> i32 {
    let s1 = a[0] as i32 + a[10] as i32;
    if let (Some(ptr1), Some(ptr2)) = (afp2[0], afp2[16]) {
        let s2 = *ptr1 as i32 + *ptr2 as i32;
        s1 + s2
    } else {
        s1
    }
}


fn main() -> i32 {
    unsafe {
        init_globals();

        if FA[0] != 1.0f32 || FA[10] != 11.0f32 {
            return 1;
        }

        if (*AFP[0].unwrap()) as i32 != 100 || (*AFP[16].unwrap()) as i32 != 116 {
            return 2;
        }

        {
            let n1 = pos(11);
            let n2 = pos(17);

            let mut vla1: Vec<f32> = vec![0.0; n1 as usize];
            let mut vla2: Vec<Option<&f32>> = vec![None; n2 as usize];

            let mut i = 0;
            while i < n1 {
                vla1[i as usize] = FA[i] * 2.0f32;
                i += 1;
            }

            i = 0;
            while i < n2 {
                vla2[i as usize] = AFP[i];
                i += 1;
            }

            if unsafe { CALLS } != 2 {
                return 3;
            }

            {
                let span = (vla1.as_ptr() as usize - vla1.as_ptr() as usize) as usize / mem::size_of::<f32>();
                if span != (n1 - 1) as usize * mem::size_of::<f32>() as usize {
                    return 4;
                }
            }

            if sum_ints_from_float(&vla1, n1) !=
                (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
                return 5;
            }

            if sum_pointed_ints(&vla2, n2) !=
                (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 +
                 111 + 112 + 113 + 114 + 115 + 116) {
                return 6;
            }

            if takes_params(&vla1, &vla2) !=
                ((vla1[0] as i32) + (vla1[10] as i32) +
                 if let Some(ptr) = vla2[0] { *ptr as i32 } else { 0 } +
                 if let Some(ptr) = vla2[16] { *ptr as i32 } else { 0 }) {
                return 7;
            }
        }

        0
    }
}