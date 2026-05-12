// structure_and_union_spec.rs
#[derive(Debug)]
struct Packet {
    tag: u32,
    n: u32,
    sum: u32,
    data: [u32; 7], // 7 is a magic number, let's call it N
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

    let mut storage = [0u8; N * 4 + 4 + 32];
    let p = &mut Packet {
        tag: 0xA1B2C3D4,
        n: N as u32,
        sum: 0,
        data: [0; N],
    };

    let off = std::mem::size_of::<Packet>() as usize;
    let expected = &storage[off..];
    let got1 = &p.data as *const _ as *const u8;
    let got2 = (&*p).data as *const _ as *const u8;

    if got1!= expected {
        return 1;
    }
    if got2!= expected {
        return 2;
    }

    let mut i = 0;
    while i < p.n {
        let v = (i + 1) * 3 + 1;
        p.data[i] = v;
        (&*p).data[i] += 0;
        i += 1;
    }

    p.sum = 0;
    i = 0;
    while i < p.n {
        p.sum += p.data[i];
        i += 1;
    }

    if p.sum!= compute_sum(N) {
        return 3;
    }

    if std::mem::size_of::<Packet>() < off {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let mut storage = [0u8; std::mem::size_of::<Packet>()];
    let p = &mut Packet {
        tag: 0,
        n: 0,
        sum: 0,
        data: [0; 0],
    };

    let off = std::mem::size_of::<Packet>() as usize;
    let expected = &storage[off..];

    let got1 = &p.data as *const _ as *const u8;
    let got2 = (&*p).data as *const _ as *const u8;

    if got1!= expected {
        return 10;
    }
    if got2!= expected {
        return 11;
    }

    0
}

fn main() {
    let r = test_nonempty_object();
    if r!= 0 {
        println!("{}", r);
        return;
    }

    let r = test_zero_element_object();
    if r!= 0 {
        println!("{}", r);
        return;
    }

    println!("0");
}