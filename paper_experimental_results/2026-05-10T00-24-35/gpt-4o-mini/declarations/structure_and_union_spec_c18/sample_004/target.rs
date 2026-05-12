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
    const N: usize = 7;

    let mut p = Packet {
        tag: 0xA1B2C3D4,
        n: N as u32,
        sum: 0,
        data: vec![0; N],
    };

    let expected = &p.data as *const _ as usize;
    let got1 = &p.data as *const _ as usize;
    let got2 = (&p).data.as_ptr() as usize;

    if got1 != expected {
        return 1;
    }
    if got2 != expected {
        return 2;
    }

    for i in 0..p.n {
        let v = (i + 1) * 3 + 1;
        p.data[i as usize] = v;
    }

    p.sum = p.data.iter().sum();

    if p.sum != compute_sum(N as u32) {
        return 3;
    }

    if std::mem::size_of::<Packet>() < std::mem::size_of::<Packet>() - std::mem::size_of::<u32>() {
        return 4;
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

    let expected = &p.data as *const _ as usize;

    let got1 = &p.data as *const _ as usize;
    let got2 = (&p).data.as_ptr() as usize;

    if got1 != expected {
        return 10;
    }
    if got2 != expected {
        return 11;
    }

    0
}

fn main() {
    let r = test_nonempty_object();
    if r != 0 {
        std::process::exit(r as i32);
    }

    let r = test_zero_element_object();
    if r != 0 {
        std::process::exit(r as i32);
    }

    std::process::exit(0);
}