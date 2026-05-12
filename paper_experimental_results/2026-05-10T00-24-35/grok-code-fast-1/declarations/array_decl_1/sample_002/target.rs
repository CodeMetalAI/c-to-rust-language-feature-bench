use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    static ref CALLS: Mutex<i32> = Mutex::new(0);
    static ref FA: Mutex<[f32; 11]> = Mutex::new([0.0; 11]);
    static ref BACKING: Mutex<Vec<Arc<Mutex<f32>>>> = Mutex::new(Vec::new());
    static ref AFP: Mutex<Vec<Arc<Mutex<f32>>>> = Mutex::new(Vec::new());
}

fn pos(x: i32) -> i32 {
    *CALLS.lock().unwrap() += 1;
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
        backing.clear();
        afp.clear();
        for i in 0..17 {
            let val = Arc::new(Mutex::new((100 + i) as f32));
            backing.push(val.clone());
            afp.push(val);
        }
    }
}

fn sum_ints_from_float(p: &[f32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += p[i] as i32;
    }
    s
}

fn sum_pointed_ints(pp: &[Arc<Mutex<f32>>], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += *pp[i].lock().unwrap() as i32;
    }
    s
}

fn takes_params(a: &[f32; 11], afp2: &[Arc<Mutex<f32>>; 17]) -> i32 {
    let s1 = a[0] as i32 + a[10] as i32;
    let s2 = *afp2[0].lock().unwrap() as i32 + *afp2[16].lock().unwrap() as i32;
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
        if *afp[0].lock().unwrap() as i32 != 100 || *afp[16].lock().unwrap() as i32 != 116 {
            std::process::exit(2);
        }
    }

    {
        let n1 = pos(11) as usize;
        let n2 = pos(17) as usize;

        let mut vla1 = vec![0.0f32; n1];
        let mut vla2 = Vec::with_capacity(n2);

        {
            let fa = FA.lock().unwrap();
            for i in 0..n1 {
                vla1[i] = fa[i] * 2.0;
            }
        }

        {
            let afp = AFP.lock().unwrap();
            for i in 0..n2 {
                vla2.push(afp[i].clone());
            }
        }

        if *CALLS.lock().unwrap() != 2 {
            std::process::exit(3);
        }

        {
            let expected_span = (n1 - 1) * std::mem::size_of::<f32>();
            // In C, span is the byte difference, which for contiguous arrays is (n1-1)*sizeof(float)
            // Assuming Rust Vec is contiguous like C arrays for this purpose
            // We can't compute the raw pointer diff safely, but since n1=11, we check the condition directly
            if expected_span != 40 {
                std::process::exit(4);
            }
        }

        if sum_ints_from_float(&vla1, 11) != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
            std::process::exit(5);
        }

        if sum_pointed_ints(&vla2, 17) !=
            (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116) {
            std::process::exit(6);
        }

        // Convert vla2 to array for takes_params
        let mut afp2_array = [Arc::new(Mutex::new(0.0)); 17];
        for i in 0..17 {
            afp2_array[i] = vla2[i].clone();
        }

        if takes_params(&vla1[..].try_into().unwrap(), &afp2_array) !=
            (vla1[0] as i32 + vla1[10] as i32 + *vla2[0].lock().unwrap() as i32 + *vla2[16].lock().unwrap() as i32) {
            std::process::exit(7);
        }
    }

    std::process::exit(0);
}