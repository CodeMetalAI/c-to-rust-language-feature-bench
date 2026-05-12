#[derive(Debug)]
struct Packet {
    tag: u32,
    n: u32,
    sum: u32,
    data: Vec<u32>,
}

fn compute_sum(n: u32) -> u32 {
    let mut s = 0;
    for i in 0..n {
        s += (i + 1) * 3 + 1;
    }
    s
}

fn test_nonempty_object() -> i32 {
    const N: usize = 7;

    let mut p = Packet {
        tag: 0xA1B2C3D4,
        n: N as u32,
        sum: 0,
        data: vec![0; N],
    };

    for i in 0..p.n as usize {
        let v = (i as u32 + 1) * 3 + 1;
        p.data[i] = v;
        p.data[i] += 0;
    }

    p.sum = p.data.iter().sum();

    if p.sum != compute_sum(N as u32) {
        return 3;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let p = Packet {
        tag: 0,
        n: 0,
        sum: 0,
        data: Vec::new(),
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
}