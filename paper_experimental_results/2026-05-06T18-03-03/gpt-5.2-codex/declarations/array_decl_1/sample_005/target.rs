use std::cell::Cell;
use std::mem::size_of;
use std::process::exit;

fn pos(x: i32, calls: &Cell<i32>) -> i32 {
    calls.set(calls.get() + 1);
    if x <= 0 {
        1
    } else {
        x
    }
}

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
    let mut fa = [0f32; 11];
    let mut backing = [0f32; 17];
    let mut afp = [0usize; 17];

    init_globals(&mut fa, &mut backing, &mut afp);

    if fa[0] != 1.0f32 || fa[10] != 11.0f32 {
        exit(1);
    }

    if backing[afp[0]] as i32 != 100 || backing[afp[16]] as i32 != 116 {
        exit(2);
    }

    let calls = Cell::new(0);

    let n1 = pos(11, &calls);
    let n2 = pos(17, &calls);

    let mut vla1 = vec![0f32; n1 as usize];
    let mut vla2 = vec![0usize; n2 as usize];

    let mut i = 0;
    while i < n1 {
        vla1[i as usize] = fa[i as usize] * 2.0f32;
        i += 1;
    }

    i = 0;
    while i < n2 {
        vla2[i as usize] = afp[i as usize];
        i += 1;
    }

    if calls.get() != 2 {
        exit(3);
    }

    {
        let span = if n1 > 0 {
            (n1 as usize - 1) * size_of::<f32>()
        } else {
            0
        };
        if span != (n1 as usize - 1) * size_of::<f32>() {
            exit(4);
        }
    }

    if sum_ints_from_float(&vla1, 11) != 132 {
        exit(5);
    }

    if sum_pointed_ints(&vla2, &backing, 17) != 1836 {
        exit(6);
    }

    if takes_params(&vla1, &vla2, &backing)
        != (vla1[0] as i32 + vla1[10] as i32 + backing[vla2[0]] as i32 + backing[vla2[16]] as i32)
    {
        exit(7);
    }

    exit(0);
}