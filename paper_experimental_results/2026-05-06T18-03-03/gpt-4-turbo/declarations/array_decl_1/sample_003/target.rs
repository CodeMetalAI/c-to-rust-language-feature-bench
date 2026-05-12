static mut CALLS: i32 = 0;

fn pos(x: i32) -> usize {
    unsafe {
        CALLS += 1;
    }
    if x <= 0 {
        1
    } else {
        x as usize
    }
}

static mut FA: [f32; 11] = [0.0; 11];
static mut BACKING: [f32; 17] = [0.0; 17];
static mut AFP: [*mut f32; 17] = [std::ptr::null_mut(); 17];

fn init_globals() {
    unsafe {
        for i in 0..11 {
            FA[i] = (i + 1) as f32;
        }
        for i in 0..17 {
            BACKING[i] = (100 + i) as f32;
            AFP[i] = &mut BACKING[i];
        }
    }
}

fn sum_ints_from_float(p: &[f32]) -> i32 {
    p.iter().map(|&x| x as i32).sum()
}

fn sum_pointed_ints(pp: &[*mut f32]) -> i32 {
    pp.iter().map(|&x| unsafe { *x } as i32).sum()
}

fn takes_params(a: &[f32], afp2: &[*mut f32]) -> i32 {
    unsafe {
        (a[0] as i32 + a[10] as i32 + *afp2[0].as_ref().unwrap() as i32 + *afp2[16].as_ref().unwrap() as i32)
    }
}

fn main() {
    init_globals();

    unsafe {
        if FA[0] != 1.0 || FA[10] != 11.0 {
            std::process::exit(1);
        }

        if *AFP[0].as_ref().unwrap() != 100.0 || *AFP[16].as_ref().unwrap() != 116.0 {
            std::process::exit(2);
        }
    }

    let n1 = pos(11);
    let n2 = pos(17);

    let mut vla1 = vec![0.0f32; n1];
    let mut vla2 = vec![std::ptr::null_mut(); n2];

    unsafe {
        for i in 0..n1 {
            vla1[i] = FA[i] * 2.0;
        }

        for i in 0..n2 {
            vla2[i] = AFP[i];
        }

        if CALLS != 2 {
            std::process::exit(3);
        }

        if sum_ints_from_float(&vla1[0..11]) != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
            std::process::exit(5);
        }

        if sum_pointed_ints(&vla2) != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116) {
            std::process::exit(6);
        }

        if takes_params(&vla1, &vla2) != (vla1[0] as i32 + vla1[10] as i32 + *vla2[0].as_ref().unwrap() as i32 + *vla2[16].as_ref().unwrap() as i32) {
            std::process::exit(7);
        }
    }
}