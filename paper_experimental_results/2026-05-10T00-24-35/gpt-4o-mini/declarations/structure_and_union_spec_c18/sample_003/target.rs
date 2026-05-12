#[derive(Debug)]
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
    const N: usize = 7;

    let mut p = Packet {
        tag: 0xA1B2C3D4,
        n: N as u32,
        sum: 0,
        data: vec![0; N],
    };

    let expected_offset = std::mem::size_of::<Packet>() - std::mem::size_of::<Vec<u32>>();
    let got1 = &p.data as *const _ as *const u8;
    let got2 = &p.data as *const _ as *const u8;

    if got1 != (p.data.as_ptr() as *const u8).wrapping_add(expected_offset) {
        return Err(1);
    }
    if got2 != (p.data.as_ptr() as *const u8).wrapping_add(expected_offset) {
        return Err(2);
    }

    for i in 0..p.n {
        let v = (i + 1) * 3 + 1;
        p.data[i as usize] = v;
    }

    p.sum = p.data.iter().sum();

    if p.sum != compute_sum(N as u32) {
        return Err(3);
    }

    if std::mem::size_of::<Packet>() < expected_offset {
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

    let expected_offset = std::mem::size_of::<Packet>() - std::mem::size_of::<Vec<u32>>();
    let got1 = &p.data as *const _ as *const u8;
    let got2 = &p.data as *const _ as *const u8;

    if got1 != (p.data.as_ptr() as *const u8).wrapping_add(expected_offset) {
        return Err(10);
    }
    if got2 != (p.data.as_ptr() as *const u8).wrapping_add(expected_offset) {
        return Err(11);
    }

    Ok(())
}

fn main() {
    if let Err(r) = test_nonempty_object() {
        std::process::exit(r);
    }

    if let Err(r) = test_zero_element_object() {
        std::process::exit(r);
    }

    std::process::exit(0);
}