// restrict_def_1.rs

#[repr(C)]
struct UnionBank {
    left: [i32; 8],
    right: [i32; 8],
}

const UNION_BANK_A: UnionBank = UnionBank { left: [0; 8], right: [0; 8] };
const UNION_BANK_B: UnionBank = UnionBank { left: [0; 8], right: [0; 8] };

static C: [i32; 8] = [0; 8];

fn choose_view(u: &UnionBank, which: bool) -> *mut i32 {
    if which {
        u.right.as_mut_ptr()
    } else {
        u.left.as_mut_ptr()
    }
}

fn fill(p: *mut i32, n: usize, base: i32) {
    for i in 0..n {
        unsafe { *p.offset(i as isize) = base + i; }
    }
}

fn bump(p: *mut i32, n: usize, k: i32) {
    for i in 0..n {
        unsafe { *p.offset(i as isize) += k; }
    }
}

fn sum(p: *const i32, n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += unsafe { *p.offset(i as isize) };
    }
    s
}

fn main() {
    let which_a = 0;
    let which_b = 1;

    let a = choose_view(&UNION_BANK_A, which_a);
    let b = choose_view(&UNION_BANK_B, which_b);

    fill(a, 8, 100);
    fill(b, 8, 200);
    fill(C.as_mut_ptr(), 8, 300);

    bump(a, 8, 1);
    bump(b, 8, 2);
    bump(C.as_mut_ptr(), 8, 3);

    if sum(a, 8) != (101 + 102 + 103 + 104 + 105 + 106 + 107 + 108) {
        eprintln!("sum(a) failed");
        std::process::exit(1);
    }

    if sum(b, 8) != (202 + 203 + 204 + 205 + 206 + 207 + 208 + 209) {
        eprintln!("sum(b) failed");
        std::process::exit(2);
    }

    if sum(C.as_ptr(), 8) != (303 + 304 + 305 + 306 + 307 + 308 + 309 + 310) {
        eprintln!("sum(C) failed");
        std::process::exit(3);
    }

    if UNION_BANK_A.left[0] != 101 {
        eprintln!("UNION_BANK_A.left[0] failed");
        std::process::exit(4);
    }

    if UNION_BANK_B.right[7] != 209 {
        eprintln!("UNION_BANK_B.right[7] failed");
        std::process::exit(5);
    }

    if C[0] != 303 || C[7] != 310 {
        eprintln!("C[0] or C[7] failed");
        std::process::exit(6);
    }

    println!("passed!");
}