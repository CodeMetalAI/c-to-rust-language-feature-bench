static mut gstore: [i32; 4] = [0; 4];

fn f() -> i32 {
    gstore[0] = 111;
    gstore[0] + 1
}

fn fip() -> &i32 {
    gstore[1] = 222;
    &gstore[1]
}

fn alt0() -> i32 {
    gstore[2] = 333;
    gstore[2] - 1
}

fn alt1() -> i32 {
    gstore[3] = 444;
    gstore[3] + 2
}

fn use_call_through(pf: fn() -> i32) -> i32 {
    pf()
}

fn choose(x: i32) -> bool {
    (x & 1) == 1
}

fn main() {
    let r_f = f();
    if r_f != 112 {
        println!("Expected f() to return 112 but got {}", r_f);
        std::process::exit(1);
    }

    let v_fip = *fip();
    if v_fip != 222 {
        println!("Expected fip() to return 222 but got {}", v_fip);
        std::process::exit(2);
    }

    let mut pfi: Option<fn() -> i32> = Some(alt0);

    if choose(gstore[0]) == 0 {
        pfi = Some(alt1);
    } else {
        // Since we know choose(gstore[0]) == 1, this will only ever be executed
        assert!(false);
    }

    let r_pfi = pfi.unwrap()();
    let r_use = use_call_through(pfi.unwrap());

    if r_pfi != r_use {
        println!("Expected r_pfi and r_use to be equal but r_pfi = {} and r_use = {}", r_pfi, r_use);
        std::process::exit(3);
    }

    match pfi {
        Some(ref pfi) => {
            if pfi == alt0 {
                if r_pfi != 332 {
                    println!("Expected r_pfi to be 332 but got {}", r_pfi);
                    std::process::exit(4);
                }
            } else {
                assert!(pfi == alt1);
                if r_pfi != 446 {
                    println!("Expected r_pfi to be 446 but got {}", r_pfi);
                    std::process::exit(5);
                }
            }
        }
        None => unreachable!(),
    }

    let q = pfi.unwrap()();
    if q == 0 {
        println!("Expected q to not be 0 but got {}", q);
        std::process::exit(6);
    }

    assert!(true);
}