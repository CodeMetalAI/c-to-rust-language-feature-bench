use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;

static CALLS: AtomicI32 = AtomicI32::new(0);
static FA: Mutex<[f32; 11]> = Mutex::new([0.0; 11]);
static BACKING: Mutex<[f32; 17]> = Mutex::new([0.0; 17]);
static AFP: Mutex<[&'static f32; 17]> = Mutex::new([&0.0; 17]);

fn pos(x: i32) -> i32 {
    CALLS.fetch_add(1, Ordering::Relaxed);
    if x <= 0 {
        1
    } else {
        x
    }
}

fn init_globals() {
    {
        let mut fa = FA.lock().unwrap();
        for i in 0..11 {
            fa[i] = (i + 1) as f32;
        }
    }
    {
        let mut backing = BACKING.lock().unwrap();
        let mut afp = AFP.lock().unwrap();
        for i in 0..17 {
            backing[i] = (100 + i) as f32;
            afp[i] = &backing[i];
        }
    }
}

fn sum_ints_from_float(p: &[f32], n: i32) -> i32 {
    let mut s = 0;
    for i in 0..n as usize {
        s += p[i] as i32;
    }
    s
}

fn sum_pointed_ints(pp: &[&f32], n: i32) -> i32 {
    let mut s = 0;
    for i in 0..n as usize {
        s += *pp[i] as i32;
    }
    s
}

fn takes_params(a: &[f32; 11], afp2: &[&f32; 17]) -> i32 {
    let s1 = a[0] as i32 + a[10] as i32;
    let s2 = *afp2[0] as i32 + *afp2[16] as i32;
    s1 + s2
}

fn main() {
    init_globals();

    {
        let fa = FA.lock().unwrap();
        if fa[0] != 1.0 || fa[10] != 11.0 {
            std::process::exit(1);
        }
    }

    {
        let afp = AFP.lock().unwrap();
        if *afp[0] as i32 != 100 || *afp[16] as i32 != 116 {
            std::process::exit(2);
        }
    }

    {
        let n1 = pos(11);
        let n2 = pos(17);

        let mut vla1: [f32; 11] = [0.0; 11];
        let mut vla2: [&f32; 17] = [&0.0; 17];

        for i in 0..n1 as usize {
            let fa = FA.lock().unwrap();
            vla1[i] = fa[i] * 2.0;
        }

        {
            let afp = AFP.lock().unwrap();
            for i in 0..n2 as usize {
                vla2[i] = afp[i];
            }
        }

        if CALLS.load(Ordering::Relaxed) != 2 {
            std::process::exit(3);
        }

        {
            let span = std::mem::size_of_val(&vla1) - std::mem::size_of::<f32>();
            if span != ((n1 - 1) as usize) * std::mem::size_of::<f32>() {
                std::process::exit(4);
            }
        }

        if sum_ints_from_float(&vla1, 11) != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
            std::process::exit(5);
        }

        if sum_pointed_ints(&vla2, 17) != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116) {
            std::process::exit(6);
        }

        if takes_params(&vla1, &vla2) != (vla1[0] as i32 + vla1[10] as i32 + *vla2[0] as i32 + *vla2[16] as i32) {
            std::process::exit(7);
        }
    }

    std::process::exit(0);
}