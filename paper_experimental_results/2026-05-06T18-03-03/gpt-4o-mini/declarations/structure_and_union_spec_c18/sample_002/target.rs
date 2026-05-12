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

fn test_nonempty_object() -> Result<(), i32> {
    const N: u32 = 7;

    let mut p = Packet {
        tag: 0xA1B2C3D4,
        n: N,
        sum: 0,
        data: vec![0; N as usize],
    };

    let expected = &p.data as *const _ as usize;
    let got1 = &p.data as *const _ as usize;
    let got2 = &p.data as *const _ as usize;

    if got1 != expected {
        return Err(1);
    }
    if got2 != expected {
        return Err(2);
    }

    for i in 0..p.n {
        let v = (i + 1) * 3 + 1;
        p.data[i as usize] = v;
    }

    p.sum = p.data.iter().sum();

    if p.sum != compute_sum(N) {
        return Err(3);
    }

    if std::mem::size_of::<Packet>() < std::mem::size_of::<Packet>() - std::mem::size_of::<Vec<u32>>() {
        return Err(4);
    }

    Ok(())
}

fn test_zero_element_object() -> Result<(), i32> {
    let p = Packet {
        tag: 0,
        n: 0,
        sum: 0,
        data: Vec::new(),
    };

    let expected = &p.data as *const _ as usize;

    let got1 = &p.data as *const _ as usize;
    let got2 = &p.data as *const _ as usize;

    if got1 != expected {
        return Err(10);
    }
    if got2 != expected {
        return Err(11);
    }

    Ok(())
}

fn main() {
    let r = test_nonempty_object().err().unwrap_or(0);
    if r != 0 {
        std::process::exit(r);
    }

    let r = test_zero_element_object().err().unwrap_or(0);
    if r != 0 {
        std::process::exit(r);
    }

    std::process::exit(0);
}