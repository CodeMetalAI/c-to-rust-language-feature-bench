fn main() {
    let mut mem = [0u8; 12];

    // u.nf.type = 1;
    mem[0..4].copy_from_slice(&1i32.to_le_bytes());

    // u.nf.doublenode = 3.14;
    mem[4..12].copy_from_slice(&3.14f64.to_le_bytes());

    // if (u.n.alltypes != 1)
    let alltypes = i32::from_le_bytes(mem[0..4].try_into().unwrap());
    if alltypes != 1 {
        std::process::exit(1);
    }

    // if (u.ni.type != 1)
    let ni_type = i32::from_le_bytes(mem[0..4].try_into().unwrap());
    if ni_type != 1 {
        std::process::exit(1);
    }

    // if (u.nf.type != 1)
    let nf_type = i32::from_le_bytes(mem[0..4].try_into().unwrap());
    if nf_type != 1 {
        std::process::exit(2);
    }

    // if (u.nf.doublenode != 3.14)
    let doublenode = f64::from_le_bytes(mem[4..12].try_into().unwrap());
    if doublenode != 3.14 {
        std::process::exit(3);
    }

    std::process::exit(0);
}