fn main() {
    // Equivalent of `static volatile int calls;`
    static mut CALLS: i32 = 0;

    fn pos(x: i32) -> i32 {
        unsafe { CALLS += 1; }
        if x <= 0 {
            return 1;
        }
        x
    }

    // Global arrays
    static mut FA: [f32; 11] = [0.0; 11];
    static mut AFP: [*mut f32; 17] = [std::ptr::null_mut(); 17];
    static mut BACKING: [f32; 17] = [0.0; 17];

    fn init_globals() {
        for i in 0..11 {
            FA[i] = (i + 1) as f32;
        }
        for i in 0..17 {
            BACKING[i] = (100 + i) as f32;
            AFP[i] = &mut BACKING[i];
        }
    }

    fn sum_ints_from_float(p: &[f32], n: usize) -> i32 {
        let mut s = 0;
        for i in 0..n {
            s += p[i] as i32;
        }
        s
    }

    fn sum_pointed_ints(pp: &[*mut f32], n: usize) -> i32 {
        let mut s = 0;
        for i in 0..n {
            // Dereference the raw pointer safely (we know it's valid)
            s += unsafe { *pp[i] } as i32;
        }
        s
    }

    fn takes_params(a: &[f32], afp2: &[*mut f32]) -> i32 {
        let s1 = a[0] as i32 + a[10] as i32;
        let s2 = unsafe { *afp2[0] } as i32 + unsafe { *afp2[16] } as i32;
        s1 + s2
    }

    init_globals();

    if FA[0] != 1.0f32 || FA[10] != 11.0f32 {
        return std::process::exit(1);
    }

    if unsafe { *AFP[0] } as i32 != 100 || unsafe { *AFP[16] } as i32 != 116 {
        return std::process::exit(2);
    }

    let n1 = pos(11);
    let n2 = pos(17);

    // Simulate VLAs with Vec
    let mut vla1 = Vec::with_capacity(n1);
    for i in 0..n1 {
        vla1.push(FA[i] * 2.0f32);
    }

    let mut vla2 = Vec::with_capacity(n2);
    for i in 0..n2 {
        vla2.push(AFP[i]);
    }

    if unsafe { CALLS } != 2 {
        return std::process::exit(3);
    }

    // Span calculation: VLAs are contiguous in memory, Vec is also contiguous
    // The original span check is essentially checking that the array is properly laid out.
    // For Vec, we can compute the difference between the addresses of the first and last elements.
    let span = unsafe {
        (vla1[n1 - 1] as *const f32 as usize) - (vla1[0] as *const f32 as usize)
    };
    if span != (n1 - 1) * std::mem::size_of::<f32>() {
        return std::process::exit(4);
    }

    if sum_ints_from_float(&vla1, 11) != (2 + 4 + 6 + 8 +10 +12 +14 +16 +18 +20 +22) {
        return std::process::exit(5);
    }

    if sum_pointed_ints(&vla2, 17) != (100 +101 +102 +103 +104 +105 +106 +107 +108 +109 +110 +111 +112 +113 +114 +115 +116) {
        return std::process::exit(6);
    }

    if takes_params(&vla1, &vla2) != (vla1[0] as i32 + vla1[10] as i32 + unsafe { *vla2[0] } as i32 + unsafe { *vla2[16] } as i32) {
        return std::process::exit(7);
    }

    std::process::exit(0);
}