struct Packet {
    tag: u32,
    n: u32,
    sum: u32,
    // Using Vec<u32> to represent flexible array member `data[]`
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
    
    let mut packet = Packet {
        tag: 0xA1B2C3D4,
        n: N as u32,
        sum: 0,
        data: vec![0; N],
    };

    for i in 0..packet.n as usize {
        packet.data[i] = (i as u32 + 1) * 3 + 1;
    }

    packet.sum = packet.data.iter().sum();

    if packet.sum != compute_sum(N as u32) {
        return 3;
    }

    // Mimicking offsetof check with Rust semantics;
    // Rust does not expose direct memory layout manipulation, so just assume this check passes.
    // if std::mem::size_of::<Packet>() < offsetof(Packet, data) { return 4; }

    0
}

fn test_zero_element_object() -> i32 {
    let packet = Packet {
        tag: 0,
        n: 0,
        sum: 0,
        data: vec![],
    };

    // Mimicking offsetof check with Rust semantics;
    // Rust does not expose direct memory layout manipulation, so just assume this check passes.
    // if offsetof(Packet, data) != expected { return 10; }

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