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
    let mut bank_a = [0i32; 8];
    let mut bank_b = [0i32; 8];
    let mut c = [0i32; 8];

    let a = &mut bank_a;
    let b = &mut bank_b;

    fill(a, 100);
    fill(b, 200);
    fill(&mut c, 300);

    bump(a, 1);
    bump(b, 2);
    bump(&mut c, 3);

    if sum(a) != (101 + 102 + 103 + 104 + 105 + 106 + 107 + 108) {
        std::process::exit(1);
    }

    if sum(b) != (202 + 203 + 204 + 205 + 206 + 207 + 208 + 209) {
        std::process::exit(2);
    }

    if sum(&c) != (303 + 304 + 305 + 306 + 307 + 308 + 309 + 310) {
        std::process::exit(3);
    }

    if bank_a[0] != 101 {
        std::process::exit(4);
    }

    if bank_b[7] != 209 {
        std::process::exit(5);
    }

    if c[0] != 303 || c[7] != 310 {
        std::process::exit(6);
    }

    std::process::exit(0);
}