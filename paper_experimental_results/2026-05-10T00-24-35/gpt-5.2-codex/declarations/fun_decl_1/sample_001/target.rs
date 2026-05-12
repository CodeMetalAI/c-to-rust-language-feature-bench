use std::cell::Cell;

thread_local! {
    static GSTORE: Cell<[i32; 4]> = Cell::new([0; 4]);
}

fn set_g(idx: usize, val: i32) {
    GSTORE.with(|c| {
        let mut arr = c.get();
        arr[idx] = val;
        c.set(arr);
    });
}

fn get_g(idx: usize) -> i32 {
    GSTORE.with(|c| {
        let arr = c.get();
        arr[idx]
    })
}

fn f() -> i32 {
    set_g(0, 111);
    get_g(0) + 1
}

fn fip() -> i32 {
    set_g(1, 222);
    get_g(1)
}

fn alt0() -> i32 {
    set_g(2, 333);
    get_g(2) - 1
}

fn alt1() -> i32 {
    set_g(3, 444);
    get_g(3) + 2
}

fn use_call_through(pf: fn() -> i32) -> i32 {
    pf()
}

fn choose(x: i32) -> i32 {
    if (x & 1) != 0 { 1 } else { 0 }
}

fn main() {
    let mut pfi: fn() -> i32;

    let r_f = f();
    if r_f != 112 {
        std::process::exit(1);
    }

    let v_fip = fip();
    if v_fip != 222 {
        std::process::exit(2);
    }

    pfi = alt0;
    if choose(get_g(0)) == 0 {
        pfi = alt1;
    }

    {
        let r_pfi = pfi();
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
        let q = pfi();
        if q == 0 {
            std::process::exit(6);
        }
    }

    std::process::exit(0);
}