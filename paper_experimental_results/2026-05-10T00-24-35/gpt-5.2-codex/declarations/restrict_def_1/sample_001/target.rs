use std::process::exit;

struct Bank {
    data: [i32; 8],
}

impl Bank {
    fn left(&self) -> &[i32; 8] {
        &self.data
    }
    fn right(&self) -> &[i32; 8] {
        &self.data
    }
    fn left_mut(&mut self) -> &mut [i32; 8] {
        &mut self.data
    }
    fn right_mut(&mut self) -> &mut [i32; 8] {
        &mut self.data
    }
}

fn choose_view<'a>(u: &'a mut Bank, which: i32) -> &'a mut [i32; 8] {
    if which != 0 {
        u.right_mut()
    } else {
        u.left_mut()
    }
}

fn fill(p: &mut [i32], base: i32) {
    let mut i = 0;
    while i < p.len() {
        p[i] = base + i as i32;
        i += 1;
    }
}

fn bump(p: &mut [i32], k: i32) {
    let mut i = 0;
    while i < p.len() {
        p[i] += k;
        i += 1;
    }
}

fn sum(p: &[i32]) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < p.len() {
        s += p[i];
        i += 1;
    }
    s
}

fn main() {
    let which_a = 0;
    let which_b = 1;

    let mut bank_a = Bank { data: [0; 8] };
    let mut bank_b = Bank { data: [0; 8] };
    let mut c = [0i32; 8];

    let a = choose_view(&mut bank_a, which_a);
    let b = choose_view(&mut bank_b, which_b);

    fill(a, 100);
    fill(b, 200);
    fill(&mut c, 300);

    bump(a, 1);
    bump(b, 2);
    bump(&mut c, 3);

    if sum(&*a) != (101 + 102 + 103 + 104 + 105 + 106 + 107 + 108) {
        exit(1);
    }

    if sum(&*b) != (202 + 203 + 204 + 205 + 206 + 207 + 208 + 209) {
        exit(2);
    }

    if sum(&c) != (303 + 304 + 305 + 306 + 307 + 308 + 309 + 310) {
        exit(3);
    }

    if bank_a.left()[0] != 101 {
        exit(4);
    }

    if bank_b.right()[7] != 209 {
        exit(5);
    }

    if c[0] != 303 || c[7] != 310 {
        exit(6);
    }

    exit(0);
}