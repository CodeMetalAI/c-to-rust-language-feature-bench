use std::mem;

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

fn sum_ints_from_float(p: &[f32], n: usize) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += p[i] as i32;
        i += 1;
    }
    s
}

fn sum_pointed_ints(pp: &[*mut f32], n: usize) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        unsafe {
            s += (*pp[i]) as i32;
        }
        i += 1;
    }
    s
}

fn takes_params(a: &[f32; 11], afp2: &[*mut f32; 17]) -> i32 {
    let s1 = a[0] as i32 + a[10] as i32;
    unsafe {
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
    }

    let n1 = pos(11);
    let n2 = pos(17);

    let mut vla1 = vec![0.0f32; n1 as usize];
    let mut vla2 = vec![std::ptr::null_mut::<f32>(); n2 as usize];

    let mut i = 0;
    while i < n1 as usize {
        unsafe {
            vla1[i] = FA[i] * 2.0f32;
        }
        i += 1;
    }

    i = 0;
    while i < n2 as usize {
        unsafe {
            vla2[i] = AFP[i];
        }
        i += 1;
    }

    unsafe {
        if CALLS != 2 {
            std::process::exit(3);
        }
    }

    {
        let span = (vla1.as_ptr().add(n1 as usize - 1) as usize)
            .wrapping_sub(vla1.as_ptr() as usize);
        if span != (n1 as usize - 1) * mem::size_of::<f32>() {
            std::process::exit(4);
        }
    }

    let sum1 = sum_ints_from_float(&vla1, 11);
    if sum1 != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
        std::process::exit(5);
    }

    let sum2 = sum_pointed_ints(&vla2, 17);
    if sum2 != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 +
                112 + 113 + 114 + 115 + 116) {
        std::process::exit(6);
    }

    let vla1_array: [f32; 11] = vla1.try_into().unwrap();
    let vla2_array: [*mut f32; 17] = vla2.try_into().unwrap();
    let result = takes_params(&vla1_array, &vla2_array);
    let expected = vla1[0] as i32 + vla1[10] as i32 + 
                   unsafe { (*vla2[0]) as i32 + (*vla2[16]) as i32 };
    if result != expected {
        std::process::exit(7);
    }

    std::process::exit(0);
}