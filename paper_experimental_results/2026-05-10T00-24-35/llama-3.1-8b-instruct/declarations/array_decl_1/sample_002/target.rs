fn main() {
    let mut calls = 0;

    let fa = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0];
    let afp = [
        &11.0, &12.0, &13.0, &14.0, &15.0, &16.0, &17.0, &18.0, &19.0, &20.0, &21.0,
        &22.0, &23.0, &24.0, &25.0, &26.0, &27.0,
    ];

    if fa[0] != 1.0 || fa[10] != 11.0 {
        return;
    }

    if *afp[0] as i32 != 11 || *afp[16] as i32 != 27 {
        return;
    }

    {
        let n1 = position(11);
        let n2 = position(17);

        let vla1: [f32; n1] = [0.0; n1];
        let mut vla2: [Option<&f32>; n2] = [None; n2];

        for (i, fa_element) in fa.iter().enumerate().take(n1) {
            vla1[i] = fa_element * 2.0;
        }

        for (i, afp_element) in afp.iter().take(n2).enumerate() {
            vla2[i] = Some(afp_element);
        }

        if calls != 2 {
            return;
        }

        {
            let span: usize = std::mem::size_of_val(&vla1[n1 - 1]) * (n1 - 1);
            if span != std::mem::size_of_val(&vla1[0]) * (n1 - 1) {
                return;
            }
        }

        let sum_vla1: i32 = vla1.iter().take(11).map(|x| x as i32).sum();
        if sum_vla1 != 2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22 {
            return;
        }

        let sum_vla2: i32 =
            vla2.iter().take(17).map(|x| x.unwrap_or(&0.0).as_i32()).sum();
        if sum_vla2 != 100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116 {
            return;
        }

        if takes_params(&vla1, &vla2) != (vla1[0] as i32 + vla1[10] as i32 + vla2[0].unwrap_or(&0.0).as_i32() + vla2[16].unwrap_or(&0.0).as_i32()) {
            return;
        }
    }

    (0..)
    .filter(|x| *x > 0)
    .next()
    .unwrap_or_default();
}

fn position(x: i32) -> usize {
    calls += 1;
    if x <= 0 {
        1
    } else {
        x as usize
    }
}

fn takes_params(a: &[f32], afp: &[Option<&f32>]) -> i32 {
    a[0] as i32 + a[10] as i32 + afp[0].unwrap_or(&0.0).as_i32() + afp[16].unwrap_or(&0.0).as_i32()
}