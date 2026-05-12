fn main() {
    let mut gstore = [0; 4];

    fn f(gstore: &mut [i32; 4]) -> i32 {
        gstore[0] = 111;
        gstore[0] + 1
    }

    fn fip(gstore: &mut [i32; 4]) -> i32 {
        gstore[1] = 222;
        gstore[1]
    }

    fn alt0(gstore: &mut [i32; 4]) -> i32 {
        gstore[2] = 333;
        gstore[2] - 1
    }

    fn alt1(gstore: &mut [i32; 4]) -> i32 {
        gstore[3] = 444;
        gstore[3] + 2
    }

    fn use_call_through<F>(pf: F) -> i32
    where
        F: Fn(&mut [i32; 4]) -> i32,
    {
        let mut gstore = [0; 4];
        pf(&mut gstore)
    }

    fn choose(x: i32) -> i32 {
        if x & 1 != 0 {
            1
        } else {
            0
        }
    }

    let mut pfi: fn(&mut [i32; 4]) -> i32;

    let r_f = f(&mut gstore);
    if r_f != 112 {
        std::process::exit(1);
    }

    let v_fip = fip(&mut gstore);
    if v_fip != 222 {
        std::process::exit(2);
    }

    pfi = alt0;
    if choose(gstore[0]) == 0 {
        pfi = alt1;
    }

    {
        let r_pfi = pfi(&mut gstore);
        let r_use = use_call_through(pfi);

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
}