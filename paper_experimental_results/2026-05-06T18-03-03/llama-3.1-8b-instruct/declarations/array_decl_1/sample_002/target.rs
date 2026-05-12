// array_decl_1.rs

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

const FA_SIZE: usize = 11;
const AFP_SIZE: usize = 17;

static mut FA: [f32; FA_SIZE] = [0.0; FA_SIZE];
static mut AFP: [Option<&mut f32>; AFP_SIZE] = [None; AFP_SIZE];

static mut BACKING: [f32; AFP_SIZE] = [0.0; AFP_SIZE];

static mut INIT_GLOBALS_DONE: bool = false;

fn init_globals() {
    if!INIT_GLOBALS_DONE {
        let mut i = 0;
        while i < FA_SIZE {
            FA[i] = (i + 1) as f32;
            i += 1;
        }
        i = 0;
        while i < AFP_SIZE {
            BACKING[i] = (100 + i) as f32;
            AFP[i] = Some(&mut BACKING[i]);
            i += 1;
        }
        INIT_GLOBALS_DONE = true;
    }
}

fn sum_ints_from_float(p: &[f32]) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < p.len() {
        s += p[i] as i32;
        i += 1;
    }
    s
}

fn sum_pointed_ints(pp: &[Option<&f32>]) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < pp.len() {
        if let Some(p) = pp[i] {
            s += p as i32;
        }
        i += 1;
    }
    s
}

fn takes_params(a: &[f32], afp2: &[Option<&f32>]) -> i32 {
    let s1 = (a[0] as i32) + (a[10] as i32);
    let s2 = if let Some(p) = afp2[0] {
        (p as i32)
    } else {
        0
    } + if let Some(p) = afp2[16] {
        (p as i32)
    } else {
        0
    };
    s1 + s2
}

fn main() {
    init_globals();

    if FA[0]!= 1.0 || FA[10]!= 11.0 {
        return 1;
    }

    if *AFP[0].unwrap()!= 100.0 || *AFP[16].unwrap()!= 116.0 {
        return 2;
    }

    {
        let n1 = pos(11);
        let n2 = pos(17);

        let mut vla1: [f32; n1] = [0.0; n1];
        let mut vla2: [Option<&f32>; n2] = [None; n2];

        let mut i = 0;
        while i < n1 {
            vla1[i] = FA[i] * 2.0;
            i += 1;
        }

        i = 0;
        while i < n2 {
            vla2[i] = Some(&BACKING[i]);
            i += 1;
        }

        if unsafe { CALLS }!= 2 {
            return 3;
        }

        {
            let span = (unsafe { &vla1[n1 - 1] as *const f32 as usize })
                - (unsafe { &vla1[0] as *const f32 as usize });
            if span!= (n1 - 1) * std::mem::size_of::<f32>() {
                return 4;
            }
        }

        if sum_ints_from_float(&vla1)!= (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
            return 5;
        }

        if sum_pointed_ints(&vla2)!= (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116) {
            return 6;
        }

        if takes_params(&vla1, &vla2)!= ((vla1[0] as i32) + (vla1[10] as i32) + (*vla2[0].unwrap() as i32) + (*vla2[16].unwrap() as i32)) {
            return 7;
        }
    }

    return 0;
}