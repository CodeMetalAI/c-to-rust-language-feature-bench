use std::cell::{RefCell, RefMut};

struct Bank {
    data: [i32; 8],
}

static BANK_A: RefCell<Bank> = RefCell::new(Bank { data: [0; 8] });
static BANK_B: RefCell<Bank> = RefCell::new(Bank { data: [0; 8] });
static C: RefCell<[i32; 8]> = RefCell::new([0; 8]);

fn choose_view(bank: &RefCell<Bank>, which: i32) -> RefMut<[i32]> {
    RefMut::map(bank.borrow_mut(), |b| b.data.as_mut_slice())
}

fn fill(p: &mut [i32], n: usize, base: i32) {
    for i in 0..n {
        p[i] = base + i as i32;
    }
}

fn bump(p: &mut [i32], n: usize, k: i32) {
    for i in 0..n {
        p[i] += k;
    }
}

fn sum(p: &[i32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += p[i];
    }
    s
}

fn main() {
    let which_a = 0;
    let which_b = 1;

    let mut a = choose_view(&BANK_A, which_a);
    let mut b = choose_view(&BANK_B, which_b);

    fill(&mut *a, 8, 100);
    fill(&mut *b, 8, 200);
    fill(&mut *C.borrow_mut(), 8, 300);

    bump(&mut *a, 8, 1);
    bump(&mut *b, 8, 2);
    bump(&mut *C.borrow_mut(), 8, 3);

    if sum(&*a, 8) != (101 + 102 + 103 + 104 + 105 + 106 + 107 + 108) {
        std::process::exit(1);
    }

    if sum(&*b, 8) != (202 + 203 + 204 + 205 + 206 + 207 + 208 + 209) {
        std::process::exit(2);
    }

    if sum(&*C.borrow(), 8) != (303 + 304 + 305 + 306 + 307 + 308 + 309 + 310) {
        std::process::exit(3);
    }

    if BANK_A.borrow().data[0] != 101 {
        std::process::exit(4);
    }

    if BANK_B.borrow().data[7] != 209 {
        std::process::exit(5);
    }

    let c = C.borrow();
    if c[0] != 303 || c[7] != 310 {
        std::process::exit(6);
    }

    std::process::exit(0);
}