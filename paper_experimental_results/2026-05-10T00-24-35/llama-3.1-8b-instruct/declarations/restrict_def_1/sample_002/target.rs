union Bank {
    left: [i32; 8],
    right: [i32; 8],
}

static mut bank_a: Bank = Bank {
    left: [0; 8],
};
static mut bank_b: Bank = Bank {
    left: [0; 8],
};
static mut c: [i32; 8] = [0; 8];

fn choose_view(u: &Bank, which: bool) -> &i32 {
    if which {
        &u.right
    } else {
        &u.left
    }
}

fn fill(p: &mut [i32], base: i32) {
    for (p, i) in p.iter_mut().zip((0..)) {
        *p = base + i;
    }
}

fn bump(p: &mut [i32], k: i32) {
    for p in p.iter_mut() {
        *p += k;
    }
}

fn sum(p: &[i32]) -> i32 {
    p.iter().sum()
}

fn main() {
    let mut which_a = 0;
    let mut which_b = 1;

    let a = choose_view(&mut bank_a, which_a);
    let b = choose_view(&mut bank_b, which_b);

    fill(a, 100);
    fill(b, 200);
    fill(&mut c, 300);

    bump(a, 1);
    bump(b, 2);
    bump(&mut c, 3);

    if sum(a) != (101 + 102 + 103 + 104 + 105 + 106 + 107 + 108) {
        return 1;
    }

    if sum(b) != (202 + 203 + 204 + 205 + 206 + 207 + 208 + 209) {
        return 2;
    }

    if sum(&c) != (303 + 304 + 305 + 306 + 307 + 308 + 309 + 310) {
        return 3;
    }

    if bank_a.left[0] != 101 {
        return 4;
    }

    if bank_b.right[7] != 209 {
        return 5;
    }

    if c[0] != 303 || c[7] != 310 {
        return 6;
    }

    0
}