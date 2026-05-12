use std::cell::{Cell, RefCell};

thread_local! {
    static CALLS: Cell<i32> = Cell::new(0);
    static FA: RefCell<[f32; 11]> = RefCell::new([0.0; 11]);
    static AFP: RefCell<[usize; 17]> = RefCell::new([0; 17]);
    static BACKING: RefCell<[f32; 17]> = RefCell::new([0.0; 17]);
}

fn pos(x: i32) -> i32 {
    CALLS.with(|c| c.set(c.get() + 1));
    if x <= 0 {
        1
    } else {
        x
    }
}

fn init_globals() {
    FA.with(|fa| {
        let mut fa = fa.borrow_mut();
        let mut i = 0;
        while i < 11 {
            fa[i] = (i + 1) as f32;
            i += 1;
        }
    });
    BACKING.with(|backing| {
        let mut backing = backing.borrow_mut();
        AFP.with(|afp| {
            let mut afp = afp.borrow_mut();
            let mut i = 0;
            while i < 17 {
                backing[i] = (100 + i) as f32;
                afp[i] = i;
                i += 1;
            }
        });
    });
}

fn sum_ints_from_float(p: &[f32], n: usize) -> i32 {
    let mut s = 0i32;
    let mut i = 0usize;
    while i < n {
        s += p[i] as i32;
        i += 1;
    }
    s
}

fn sum_pointed_ints(pp: &[usize], n: usize, backing: &[f32]) -> i32 {
    let mut s = 0i32;
    let mut i = 0usize;
    while i < n {
        let idx = pp[i];
        s += backing[idx] as i32;
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
    init_globals();

    let fa_ok = FA.with(|fa| {
        let fa = fa.borrow();
        fa[0] == 1.0f32 && fa[10] == 11.0f32
    });
    if !fa_ok {
        std::process::exit(1);
    }

    let afp_ok = BACKING.with(|backing| {
        let backing = backing.borrow();
        AFP.with(|afp| {
            let afp = afp.borrow();
            backing[afp[0]] as i32 == 100 && backing[afp[16]] as i32 == 116
        })
    });
    if !afp_ok {
        std::process::exit(2);
    }

    let n1 = pos(11) as usize;
    let n2 = pos(17) as usize;

    let mut vla1 = vec![0.0f32; n1];
    let mut vla2 = vec![0usize; n2];

    FA.with(|fa| {
        let fa = fa.borrow();
        let mut i = 0usize;
        while i < n1 {
            vla1[i] = fa[i] * 2.0f32;
            i += 1;
        }
    });

    AFP.with(|afp| {
        let afp = afp.borrow();
        let mut i = 0usize;
        while i < n2 {
            vla2[i] = afp[i];
            i += 1;
        }
    });

    let calls_ok = CALLS.with(|c| c.get() == 2);
    if !calls_ok {
        std::process::exit(3);
    }

    let span = (n1 - 1) * std::mem::size_of::<f32>();
    if span != (n1 - 1) * std::mem::size_of::<f32>() {
        std::process::exit(4);
    }

    if sum_ints_from_float(&vla1, 11)
        != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22)
    {
        std::process::exit(5);
    }

    let sum_pointed = BACKING.with(|backing| {
        let backing = backing.borrow();
        sum_pointed_ints(&vla2, 17, &backing)
    });
    if sum_pointed
        != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112
            + 113 + 114 + 115 + 116)
    {
        std::process::exit(6);
    }

    let takes = BACKING.with(|backing| {
        let backing = backing.borrow();
        takes_params(&vla1, &vla2, &backing)
    });
    let expected = vla1[0] as i32
        + vla1[10] as i32
        + BACKING.with(|backing| {
            let backing = backing.borrow();
            backing[vla2[0]] as i32 + backing[vla2[16]] as i32
        });
    if takes != expected {
        std::process::exit(7);
    }

    std::process::exit(0);
}