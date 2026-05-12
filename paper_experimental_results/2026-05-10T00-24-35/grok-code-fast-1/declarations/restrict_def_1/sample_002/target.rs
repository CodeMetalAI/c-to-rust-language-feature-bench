#[derive(Debug, Clone)]
struct Bank {
    data: [i32; 8],
}

impl Bank {
    fn choose_view(&mut self, which: i32) -> &mut [i32] {
        // Since left and right overlap in the union, they are the same data
        &mut self.data
    }
}

fn fill(p: &mut [i32], base: i32) {
    for i in 0..p.len() {
        p[i] = base + i as i32;
    }
}

fn bump(p: &mut [i32], k: i32) {
    for i in 0..p.len() {
        p[i] += k;
    }
}

fn sum(p: &[i32]) -> i32 {
    p.iter().sum()
}

fn main() {
    let mut bank_a = Bank { data: [0; 8] };
    let mut bank_b = Bank { data: [0; 8] };
    let mut c = [0i32; 8];

    let which_a = 0;
    let which_b = 1;

    let a = bank_a.choose_view(which_a);
    let b = bank_b.choose_view(which_b);

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

    if bank_a.data[0] != 101 {
        std::process::exit(4);
    }

    if bank_b.data[7] != 209 {
        std::process::exit(5);
    }

    if c[0] != 303 || c[7] != 310 {
        std::process::exit(6);
    }

    std::process::exit(0);
}