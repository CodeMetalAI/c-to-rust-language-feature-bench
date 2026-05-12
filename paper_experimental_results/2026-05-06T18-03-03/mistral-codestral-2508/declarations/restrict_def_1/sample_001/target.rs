#[derive(Copy, Clone)]
union Bank {
    left: [i32; 8],
    right: [i32; 8],
}

static mut BANK_A: Bank = Bank { left: [0; 8] };
static mut BANK_B: Bank = Bank { left: [0; 8] };

static mut A: *mut i32 = std::ptr::null_mut();
static mut B: *mut i32 = std::ptr::null_mut();
static mut C: [i32; 8] = [0; 8];

fn choose_view(u: &mut Bank, which: bool) -> *mut i32 {
    if which {
        unsafe { &mut u.right as *mut [i32; 8] as *mut i32 }
    } else {
        unsafe { &mut u.left as *mut [i32; 8] as *mut i32 }
    }
}

fn fill(p: *mut i32, n: usize, base: i32) {
    for i in 0..n {
        unsafe { *p.add(i) = base + i as i32 };
    }
}

fn bump(p: *mut i32, n: usize, k: i32) {
    for i in 0..n {
        unsafe { *p.add(i) += k };
    }
}

fn sum(p: *const i32, n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        unsafe { s += *p.add(i) };
    }
    s
}

fn main() {
    let which_a = false;
    let which_b = true;

    unsafe {
        A = choose_view(&mut BANK_A, which_a);
        B = choose_view(&mut BANK_B, which_b);

        fill(A, 8, 100);
        fill(B, 8, 200);
        fill(C.as_mut_ptr(), 8, 300);

        bump(A, 8, 1);
        bump(B, 8, 2);
        bump(C.as_mut_ptr(), 8, 3);

        if sum(A, 8) != (101 + 102 + 103 + 104 + 105 + 106 + 107 + 108) {
            std::process::exit(1);
        }

        if sum(B, 8) != (202 + 203 + 204 + 205 + 206 + 207 + 208 + 209) {
            std::process::exit(2);
        }

        if sum(C.as_ptr(), 8) != (303 + 304 + 305 + 306 + 307 + 308 + 309 + 310) {
            std::process::exit(3);
        }

        if BANK_A.left[0] != 101 {
            std::process::exit(4);
        }

        if BANK_B.right[7] != 209 {
            std::process::exit(5);
        }

        if C[0] != 303 || C[7] != 310 {
            std::process::exit(6);
        }
    }
}