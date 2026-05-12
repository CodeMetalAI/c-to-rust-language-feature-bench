fn main() {
    let mut calls = 0;
    let mut fa = [0.0; 11];
    let mut afp = [0.0; 17];
    let mut backing = [0.0; 17];

    init_globals(&mut fa, &mut afp, &mut backing);

    if fa[0] != 1.0 || fa[10] != 11.0 {
        return 1;
    }

    if afp[0] != 100.0 || afp[16] != 116.0 {
        return 2;
    }

    {
        let n1 = pos(11);
        let n2 = pos(17);

        let mut vla1 = [0.0; n1];
        let mut vla2: [&dyn float_ptr; n2] = Default::default();

        for (i, fa_val) in fa.iter().enumerate() {
            vla1[i] = fa_val * 2.0;
        }

        for (i, afp_val) in afp.iter().enumerate() {
            vla2[i] = afp_val;
        }

        if calls != 2 {
            return 3;
        }

        {
            let span = vla1.len() as usize * std::mem::size_of::<f64>();
            if span != vla1.len() - 1 {
                return 4;
            }
        }

        if sum_ints_from_float(&vla1[..11]) != sum(2..23) {
            return 5;
        }

        if sum_pointed_ints(&vla2[..17]) != sum(100..117) {
            return 6;
        }

        if takes_params(&vla1, &vla2) != (vla1[0] as i32 + vla1[10] as i32 + vla2[0] as i32 + vla2[16] as i32) {
            return 7;
        }
    }

    return 0;
}

fn init_globals(fa: &mut [f64], afp: &mut [f64], backing: &mut [f64]) {
    for (i, fa_val) in fa.iter_mut().enumerate() {
        *fa_val = i as f64 + 1.0;
    }

    for (i, afp_val) in afp.iter_mut().enumerate() {
        *afp_val = 100.0 + i as f64;
    }

    for i in 0..17 {
        *afp.get_mut(i).unwrap() = backing[i];
    }
}

fn pos(x: i32) -> i32 {
    calls += 1;
    if x <= 0 {
        1
    } else {
        x
    }
}

fn sum_pointed_ints(pp: &[&dyn float_ptr], n: usize) -> i32 {
    let mut s = 0;
    for val in pp[..n].iter() {
        s += val as i32;
    }
    s
}

fn sum_ints_from_float(p: &[f64], n: usize) -> i32 {
    let mut s = 0;
    for val in p[..n].iter() {
        s += val as i32;
    }
    s
}

fn takes_params(a: &[f64], afp2: &[&dyn float_ptr]) -> i32 {
    let s1 = (a[0] as i32) + (a[10] as i32);
    let s2 = (afp2[0] as i32) + (afp2[16] as i32);
    s1 + s2
}

trait float_ptr {
    fn as_i32(&self) -> i32;
}

impl float_ptr for f64 {
    fn as_i32(&self) -> i32 {
        self as i32
    }
}