use std::mem;

fn pos(x: i32) -> i32 {
    static mut CALLS: i32 = 0;
    unsafe {
        CALLS += 1;
    }
    if x <= 0 {
        1
    } else {
        x
    }
}

fn init_globals() {
    let mut fa = [0.0; 11];
    let mut backing = [0.0; 17];
    let mut afp = [0.0; 17];

    for (i, fa) in fa.iter_mut().enumerate() {
        *fa = (i + 1) as f32;
    }

    for (i, (backing, afp)) in backing.iter_mut().zip(afp.iter_mut()).enumerate() {
        *backing = (100 + i) as f32;
        *afp = &backing[i];
    }
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
        s += pp[i].clone() as i32;
    }
    s
}

fn takes_params(a: &[f32], afp2: &[&f32]) -> i32 {
    let s1 = a[0] as i32 + a[10] as i32;
    let s2 = afp2[0].clone() as i32 + afp2[16].clone() as i32;
    s1 + s2
}

fn main() {
    init_globals();

    if fa[0]!= 1.0 || fa[10]!= 11.0 {
        panic!("fa[0]!= 1.0 || fa[10]!= 11.0");
    }

    if (*afp[0])!= 100.0 || (*afp[16])!= 116.0 {
        panic!("(*afp[0])!= 100.0 || (*afp[16])!= 116.0");
    }

    {
        let n1 = pos(11);
        let n2 = pos(17);

        let mut vla1 = vec![0.0; n1];
        let mut vla2 = vec![0.0; n2];

        for (i, vla1) in vla1.iter_mut().enumerate() {
            *vla1 = fa[i] * 2.0;
        }

        for (i, vla2) in vla2.iter_mut().enumerate() {
            *vla2 = afp[i].clone();
        }

        if unsafe { CALLS }!= 2 {
            panic!("CALLS!= 2");
        }

        {
            let span = mem::size_of::<f32>() * (n1 - 1);
            if span!= (n1 - 1) * mem::size_of::<f32>() {
                panic!("span!= (n1 - 1) * mem::size_of::<f32>()");
            }
        }

        if sum_ints_from_float(&vla1, 11)!= (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
            panic!("sum_ints_from_float(&vla1, 11)!= (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22)");
        }

        if sum_pointed_ints(&vla2, 17)!= (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116) {
            panic!("sum_pointed_ints(&vla2, 17)!= (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116)");
        }

        if takes_params(&vla1, &vla2)!= ((vla1[0] as i32) + (vla1[10] as i32) + (vla2[0].clone() as i32) + (vla2[16].clone() as i32)) {
            panic!("takes_params(&vla1, &vla2)!= ((vla1[0] as i32) + (vla1[10] as i32) + (vla2[0].clone() as i32) + (vla2[16].clone() as i32))");
        }
    }

    assert!(true);
}