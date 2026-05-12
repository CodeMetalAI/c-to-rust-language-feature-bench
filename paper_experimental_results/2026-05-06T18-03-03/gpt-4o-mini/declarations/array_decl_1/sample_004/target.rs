type usize = u64;

static mut CALLS: i32 = 0;

fn pos(x: i32) -> i32 {
    unsafe { CALLS += 1; }
    if x <= 0 {
        return 1;
    }
    x
}

const FA_LEN: usize = 11;
const AFP_LEN: usize = 17;

static mut FA: [f32; FA_LEN as usize] = [0.0; FA_LEN as usize];
static mut AFP: [&'static f32; AFP_LEN as usize] = [&0.0; AFP_LEN as usize];
static mut BACKING: [f32; AFP_LEN as usize] = [0.0; AFP_LEN as usize];

fn init_globals() {
    unsafe {
        for i in 0..FA_LEN {
            FA[i as usize] = (i + 1) as f32;
        }
        for i in 0..AFP_LEN {
            BACKING[i as usize] = (100 + i) as f32;
            AFP[i as usize] = &BACKING[i as usize];
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

fn sum_pointed_ints(pp: &[&f32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += *pp[i] as i32;
    }
    s
}

fn takes_params(a: &[f32; FA_LEN as usize], afp2: &[&f32; AFP_LEN as usize]) -> i32 {
    let s1 = a[0] as i32 + a[10] as i32;
    let s2 = *afp2[0] as i32 + *afp2[16] as i32;
    s1 + s2
}

fn main() -> i32 {
    init_globals();

    unsafe {
        if FA[0] != 1.0 || FA[10] != 11.0 {
            return 1;
        }

        if (*AFP[0] as i32) != 100 || (*AFP[16] as i32) != 116 {
            return 2;
        }

        let n1 = pos(11);
        let n2 = pos(17);

        let mut vla1 = vec![0.0; n1 as usize];
        let mut vla2 = vec![std::ptr::null(); n2 as usize];

        for i in 0..n1 {
            vla1[i as usize] = FA[i as usize] * 2.0;
        }

        for i in 0..n2 {
            vla2[i as usize] = AFP[i as usize];
        }

        if CALLS != 2 {
            return 3;
        }

        let span = (vla1.as_ptr().add(n1 as usize - 1) as usize) - (vla1.as_ptr() as usize);
        if span != (n1 as usize - 1) * std::mem::size_of::<f32>() {
            return 4;
        }

        if sum_ints_from_float(&vla1, 11) != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
            return 5;
        }

        if sum_pointed_ints(&vla2, 17) != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116) {
            return 6;
        }

        if takes_params(&vla1, &vla2) != ((vla1[0] as i32) + (vla1[10] as i32) + (*vla2[0] as i32) + (*vla2[16] as i32)) {
            return 7;
        }
    }

    0
}