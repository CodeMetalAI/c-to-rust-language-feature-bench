type u8 = u8;
type u32 = u32;

struct Packet {
    tag: u32,
    n: u32,
    sum: u32,
    data: Vec<u32>,
}

fn compute_sum(n: u32) -> u32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += (i + 1) * 3 + 1;
        i += 1;
    }
    s
}

fn test_nonempty_object() -> i32 {
    const N: u32 = 7;
    let mut p = Packet {
        tag: 0xA1B2C3D4,
        n: N,
        sum: 0,
        data: vec![0; N as usize],
    };
    let mut i = 0;
    while i < p.n {
        let v = (i + 1) * 3 + 1;
        p.data[i as usize] = v;
        p.data[i as usize] += 0;
        i += 1;
    }
    p.sum = 0;
    i = 0;
    while i < p.n {
        p.sum += p.data[i as usize];
        i += 1;
    }
    if p.sum != compute_sum(N) {
        return 3;
    }
    0
}

fn test_zero_element_object() -> i32 {
    let _p = Packet {
        tag: 0,
        n: 0,
        sum: 0,
        data: vec![],
    };
    0
}

fn main() {
    let r = test_nonempty_object();
    if r != 0 {
        std::process::exit(r);
    }
    let r = test_zero_element_object();
    if r != 0 {
        std::process::exit(r);
    }
    std::process::exit(0);
}