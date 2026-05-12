type U32 = u32;

struct Packet {
    tag: U32,
    n: U32,
    sum: U32,
    data: Vec<U32>,
}

fn compute_sum(n: U32) -> U32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += (i + 1) * 3 + 1;
        i += 1;
    }
    s
}

fn test_nonempty_object() -> i32 {
    let n = 7;
    let mut p = Packet {
        tag: 0xA1B2C3D4,
        n,
        sum: 0,
        data: vec![0; n as usize],
    };
    for i in 0..p.n {
        let v = (i + 1) * 3 + 1;
        p.data[i as usize] = v;
        p.data[i as usize] += 0;
    }
    p.sum = p.data.iter().sum();
    if p.sum != compute_sum(n) {
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
    let mut r = test_nonempty_object();
    if r != 0 {
        std::process::exit(r);
    }
    r = test_zero_element_object();
    if r != 0 {
        std::process::exit(r);
    }
    std::process::exit(0);
}