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

    let expected = std::mem::size_of::<Packet>() + N as usize * std::mem::size_of::<u32>();
    
    let got1 = &p.data as *const _ as *const u8;
    let got2 = &p.data as *const _ as *const u8;

    if got1 as usize != expected as usize {
        return 1;
    }
    if got2 as usize != expected as usize {
        return 2;
    }

    for i in 0..p.n {
        let v = (i + 1) * 3 + 1;
        p.data[i as usize] = v;
    }

    for i in 0..p.n {
        p.sum += p.data[i as usize];
    }

    if p.sum != compute_sum(N) {
        return 3;
    }

    if std::mem::size_of::<Packet>() < std::mem::size_of::<Packet>() {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let p = Packet {
        tag: 0,
        n: 0,
        sum: 0,
        data: vec![],
    };

    let expected = std::mem::size_of::<Packet>();

    let got1 = &p.data as *const _ as *const u8;
    let got2 = &p.data as *const _ as *const u8;

    if got1 as usize != expected {
        return 10;
    }
    if got2 as usize != expected {
        return 11;
    }

    0
}

fn main() {
    let r;

    r = test_nonempty_object();
    if r != 0 {
        std::process::exit(r);
    }

    r = test_zero_element_object();
    if r != 0 {
        std::process::exit(r);
    }

    std::process::exit(0);
}