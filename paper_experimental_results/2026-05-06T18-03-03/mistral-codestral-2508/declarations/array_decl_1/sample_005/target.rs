static mut CALLS: i32 = 0;

fn pos(x: i32) -> i32 {
    unsafe {
        CALLS += 1;
    }
    if x <= 0 {
        1
    } else {
        x
    }
}

static mut FA: [f32; 11] = [0.0; 11];
static mut AFP: [Option<&'static mut f32>; 17] = [None; 17];
static mut BACKING: [f32; 17] = [0.0; 17];

fn init_globals() {
    unsafe {
        for i in 0..11 {
            FA[i] = (i + 1) as f32;
        }
        for i in 0..17 {
            BACKING[i] = (100 + i) as f32;
            AFP[i] = Some(&mut BACKING[i]);
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

fn sum_pointed_ints(pp: &[Option<&mut f32>], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        if let Some(val) = pp[i] {
            s += *val as i32;
        }
    }
    s
}

fn takes_params(a: &[f32; 11], afp2: &[Option<&mut f32>; 17]) -> i32 {
    let s1 = a[0] as i32 + a[10] as i32;
    let s2 = if let Some(val) = afp2[0] {
        *val as i32
    } else {
        0
    } + if let Some(val) = afp2[16] {
        *val as i32
    } else {
        0
    };
    s1 + s2
}

fn main() {
    unsafe {
        init_globals();

        if FA[0] != 1.0 || FA[10] != 11.0 {
            std::process::exit(1);
        }

        if let Some(val) = AFP[0] {
            if *val as i32 != 100 {
                std::process::exit(2);
            }
        } else {
            std::process::exit(2);
        }

        if let Some(val) = AFP[16] {
            if *val as i32 != 116 {
                std::process::exit(2);
            }
        } else {
            std::process::exit(2);
        }

        let n1 = pos(11);
        let n2 = pos(17);

        let mut vla1: Vec<f32> = vec![0.0; n1 as usize];
        let mut vla2: Vec<Option<&mut f32>> = vec![None; n2 as usize];

        for i in 0..n1 {
            vla1[i as usize] = FA[i as usize] * 2.0;
        }

        for i in 0..n2 {
            vla2[i as usize] = AFP[i as usize];
        }

        if CALLS != 2 {
            std::process::exit(3);
        }

        let span = (n1 - 1) as usize * std::mem::size_of::<f32>();
        if span != (n1 - 1) as usize * std::mem::size_of::<f32>() {
            std::process::exit(4);
        }

        if sum_ints_from_float(&vla1, 11) != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
            std::process::exit(5);
        }

        if sum_pointed_ints(&vla2, 17) != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116) {
            std::process::exit(6);
        }

        if takes_params(&vla1[..11].try_into().unwrap(), &vla2[..17].try_into().unwrap()) != (vla1[0] as i32 + vla1[10] as i32 + if let Some(val) = vla2[0] { *val as i32 } else { 0 } + if let Some(val) = vla2[16] { *val as i32 } else { 0 }) {
            std::process::exit(7);
        }
    }
}