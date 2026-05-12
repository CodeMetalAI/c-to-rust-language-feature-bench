fn f(g: &mut [i32; 4]) -> i32 {
    g[0] = 111;
    g[0] + 1
}

fn fip(g: &mut [i32; 4]) -> &mut i32 {
    g[1] = 222;
    &mut g[1]
}

fn alt0(g: &mut [i32; 4]) -> i32 {
    g[2] = 333;
    g[2] - 1
}

fn alt1(g: &mut [i32; 4]) -> i32 {
    g[3] = 444;
    g[3] + 2
}

fn use_call_through(pf: fn(&mut [i32; 4]) -> i32, g: &mut [i32; 4]) -> i32 {
    pf(g)
}

fn choose(x: i32) -> i32 {
    if x & 1 != 0 {
        1
    } else {
        0
    }
}

fn main() {
    let mut gstore = [0i32; 4];

    let r_f = f(&mut gstore);
    if r_f != 112 {
        std::process::exit(1);
    }

    let v_fip = *fip(&mut gstore);
    if v_fip != 222 {
        std::process::exit(2);
    }

    let mut pfi: fn(&mut [i32; 4]) -> i32 = alt0;
    if choose(gstore[0]) == 0 {
        pfi = alt1;
    }

    {
        let r_pfi = pfi(&mut gstore);
        let r_use = use_call_through(pfi, &mut gstore);

        if r_pfi != r_use {
            std::process::exit(3);
        }

        if pfi == alt0 {
            if r_pfi != 332 {
                std::process::exit(4);
            }
        } else {
            if r_pfi != 446 {
                std::process::exit(5);
            }
        }
    }

    {
        let q = pfi(&mut gstore);
        if q == 0 {
            std::process::exit(6);
        }
    }

    std::process::exit(0);
}