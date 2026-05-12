union Bank {
    left: [i32; 8],
    right: [i32; 8],
}

static mut bank_a: Bank = Bank { left: [0; 8] };
static mut bank_b: Bank = Bank { left: [0; 8] };

static mut c: [i32; 8] = [0; 8];

fn choose_view(u: &mut Bank, which: bool) -> *mut i32 {
    if which {
        u.right.as_mut_ptr()
    } else {
        u.left.as_mut_ptr()
    }
}

fn fill(p: *mut i32, n: usize, base: i32) {
    let mut i = 0;
    while i < n {
        unsafe { *p.offset(i as isize) = base + i; }
        i += 1;
    }
}

fn bump(p: *mut i32, n: usize, k: i32) {
    let mut i = 0;
    while i < n {
        unsafe { *p.offset(i as isize) += k; }
        i += 1;
    }
}

fn sum(p: *const i32, n: usize) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += unsafe { *p.offset(i as isize) };
        i += 1;
    }
    s
}

fn main() {
    let mut a = choose_view(&mut bank_a, false);
    let mut b = choose_view(&mut bank_b, true);

    fill(a, 8, 100);
    fill(b, 8, 200);
    fill(c.as_mut_ptr(), 8, 300);

    bump(a, 8, 1);
    bump(b, 8, 2);
    bump(c.as_mut_ptr(), 8, 3);

    if sum(a, 8)!= (101 + 102 + 103 + 104 + 105 + 106 + 107 + 108) {
        panic!("sum(a, 8) failed");
    }

    if sum(b, 8)!= (202 + 203 + 204 + 205 + 206 + 207 + 208 + 209) {
        panic!("sum(b, 8) failed");
    }

    if sum(c.as_ptr(), 8)!= (303 + 304 + 305 + 306 + 307 + 308 + 309 + 310) {
        panic!("sum(c, 8) failed");
    }

    if bank_a.left[0]!= 101 {
        panic!("bank_a.left[0] failed");
    }

    if bank_b.right[7]!= 209 {
        panic!("bank_b.right[7] failed");
    }

    if c[0]!= 303 || c[7]!= 310 {
        panic!("c[0] or c[7] failed");
    }

    println!("Success");
}