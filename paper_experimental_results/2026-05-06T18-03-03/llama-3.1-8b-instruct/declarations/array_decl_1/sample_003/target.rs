use std::mem;

fn pos(x: i32) -> i32 {
    calls += 1;
    if x <= 0 {
        1
    } else {
        x
    }
}

const FA_SIZE: usize = 11;
const AFP_SIZE: usize = 17;

static mut CALLS: i32 = 0;

static mut FA: [f32; FA_SIZE] = [0.0; FA_SIZE];
static mut AFP: [*const f32; AFP_SIZE] = [std::ptr::null(); AFP_SIZE];
static mut BACKING: [f32; AFP_SIZE] = [0.0; AFP_SIZE];

fn init_globals() {
    for i in 0..FA_SIZE {
        FA[i] = (i as f32 + 1.0);
    }
    for i in 0..AFP_SIZE {
        BACKING[i] = (100.0 + i as f32);
        AFP[i] = &BACKING[i];
    }
}

fn sum_ints_from_float(p: &[f32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += (p[i] as i32);
    }
    s
}

fn sum_pointed_ints(pp: &[*const f32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += (*pp[i] as i32);
    }
    s
}

fn takes_params(a: &[f32], afp2: &[*const f32]) -> i32 {
    let s1 = (a[0] as i32) + (a[10] as i32);
    let s2 = (*afp2[0] as i32) + (*afp2[16] as i32);
    s1 + s2
}

fn main() {
    init_globals();

    if FA[0]!= 1.0 || FA[10]!= 11.0 {
        return 1;
    }

    if (*AFP[0] as i32)!= 100 || (*AFP[16] as i32)!= 116 {
        return 2;
    }

    {
        let n1 = pos(11);
        let n2 = pos(17);

        let mut vla1: [f32; n1] = [0.0; n1];
        let mut vla2: [*const f32; n2] = [std::ptr::null(); n2];

        for i in 0..n1 {
            vla1[i] = FA[i] * 2.0;
        }

        for i in 0..n2 {
            vla2[i] = AFP[i];
        }

        unsafe {
            if CALLS!= 2 {
                return 3;
            }

            let span = (std::mem::size_of::<f32>() * (n1 - 1)) as usize;
            if span!= (n1 - 1) * std::mem::size_of::<f32>() {
                return 4;
            }
        }

        if sum_ints_from_float(&vla1, n1)!= (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
            return 5;
        }

        if sum_pointed_ints(&vla2, n2)!= (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116) {
            return 6;
        }

        if takes_params(&vla1, &vla2)!= ((vla1[0] as i32) + (vla1[10] as i32) + (*vla2[0] as i32) + (*vla2[16] as i32)) {
            return 7;
        }
    }

    return 0;
}