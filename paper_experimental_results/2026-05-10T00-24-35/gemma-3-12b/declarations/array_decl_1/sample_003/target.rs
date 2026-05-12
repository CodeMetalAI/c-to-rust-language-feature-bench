#[derive(Copy, Clone)]
struct FloatArray([f32; 11]);

fn main() {
    static mut CALLS: i32 = 0;
    static FA: FloatArray = FloatArray([0.0; 11]);
    static mut AFP: &[&f32; 17] = unsafe { &[] };
    static BACKING: [f32; 17] = [0.0; 17];

    unsafe {
        init_globals();

        if FA.0 != 1.0 || FA.10 != 11.0 {
            return 1;
        }

        if (*AFP[0]) as i32 != 100 || (*AFP[16]) as i32 != 116 {
            return 2;
        }

        let n1 = pos(11);
        let n2 = pos(17);

        let mut vla1: Vec<f32> = vec![0.0; n1];
        let mut vla2: Vec<&f32> = Vec::with_capacity(n2);

        for i in 0..n1 {
            vla1[i] = FA.0[i] * 2.0;
        }

        for i in 0..n2 {
            vla2.push(AFP[i]);
        }

        if CALLS != 2 {
            return 3;
        }

        let span = {
            let ptr = vla1.as_ptr() as *const u8;
            let end = unsafe { ptr.add(n1 as usize).add(std::mem::size_of::<f32>()) };
            (end as usize - ptr as usize) as usize
        };

        if span != (n1 as usize - 1) * std::mem::size_of::<f32>() as usize {
            return 4;
        }

        if sum_ints_from_float(&vla1, n1) != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
            return 5;
        }

        let mut sum_pointed = 0;
        for i in 0..n2 {
            sum_pointed += (*vla2[i]) as i32;
        }
        
        if sum_pointed != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116) {
            return 6;
        }

        let s1 = vla1[0] as i32 + vla1[10] as i32;
        let s2 = (*vla2[0]) as i32 + (*vla2[16]) as i32;

        if takes_params(&vla1, &vla2) != (s1 + s2) {
            return 7;
        }
    }

    0
}

fn pos(x: i32) -> i32 {
    unsafe {
        CALLS += 1;
        if x <= 0 {
            1
        } else {
            x
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
        s += (*pp[i]) as i32;
    }
    s
}

fn takes_params(a: &[f32], afp2: &[&f32]) -> i32 {
    let s1 = a[0] as i32 + a[10] as i32;
    let s2 = (*afp2[0]) as i32 + (*afp2[16]) as i32;
    s1 + s2
}

unsafe fn init_globals() {
    for i in 0..11 {
        FA.0[i] = (i + 1) as f32;
    }
    for i in 0..17 {
        BACKING[i] = (100 + i) as f32;
        AFP[i] = &BACKING[i];
    }
}