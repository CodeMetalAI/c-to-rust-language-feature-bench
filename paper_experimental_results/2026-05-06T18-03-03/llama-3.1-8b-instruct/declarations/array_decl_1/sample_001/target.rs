// array_decl_1.rs

use std::mem;

const MAX_FLA: usize = 11;
const MAX_AFP: usize = 17;

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

static mut FA: [f32; MAX_FLA] = [0.0; MAX_FLA];
static mut AFP: [Option<&mut f32>; MAX_AFP] = [None; MAX_AFP];

static mut BACKING: [f32; MAX_AFP] = [0.0; MAX_AFP];

fn init_globals() {
    for i in 0..MAX_FLA {
        FA[i] = (i as f32 + 1.0);
    }
    for i in 0..MAX_AFP {
        BACKING[i] = (100.0 + i as f32);
        AFP[i] = Some(&mut BACKING[i]);
    }
}

fn sum_ints_from_float(p: &[f32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += p[i] as i32;
    }
    s
}

fn sum_pointed_ints(pp: &[Option<&mut f32>], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        if let Some(p) = pp[i] {
            s += p as i32;
        }
    }
    s
}

fn takes_params(a: &[f32], afp2: &[Option<&mut f32>]) -> i32 {
    let s1 = a[0] as i32 + a[10] as i32;
    let s2 = afp2[0].unwrap_or(&mut 0.0) as i32 + afp2[16].unwrap_or(&mut 0.0) as i32;
    s1 + s2
}

fn main() {
    init_globals();

    if FA[0] != 1.0 || FA[10] != 11.0 {
        return 1;
    }

    if *AFP[0].unwrap_or(&mut 0.0) as i32 != 100 || *AFP[16].unwrap_or(&mut 0.0) as i32 != 116 {
        return 2;
    }

    {
        let n1 = pos(11);
        let n2 = pos(17);

        let mut vla1: Vec<f32> = vec![0.0; n1];
        let mut vla2: Vec<Option<&mut f32>> = vec![None; n2];

        for i in 0..n1 {
            vla1[i] = FA[i] * 2.0;
        }

        for i in 0..n2 {
            vla2[i] = Some(&mut BACKING[i]);
        }

        if unsafe { CALLS } != 2 {
            return 3;
        }

        {
            let span = unsafe {
                mem::size_of::<f32>() * (n1 - 1)
            };
            if span != (n1 - 1) * mem::size_of::<f32>() {
                return 4;
            }
        }

        if sum_ints_from_float(&vla1, 11) != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
            return 5;
        }

        if sum_pointed_ints(&vla2, 17) != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116) {
            return 6;
        }

        if takes_params(&vla1, &vla2) != ((vla1[0] as i32) + (vla1[10] as i32) + (*vla2[0].unwrap_or(&mut 0.0) as i32) + (*vla2[16].unwrap_or(&mut 0.0) as i32)) {
            return 7;
        }
    }

    return 0;
}