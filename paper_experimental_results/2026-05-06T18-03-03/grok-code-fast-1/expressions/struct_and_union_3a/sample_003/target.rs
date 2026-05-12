fn main() {
    let mut data = [0u8; 12];

    // Set u.nf.type (first 4 bytes)
    data[0..4].copy_from_slice(&1i32.to_ne_bytes());

    // Set u.nf.doublenode (next 8 bytes)
    data[4..12].copy_from_slice(&3.14f64.to_ne_bytes());

    // Check u.n.alltypes (first 4 bytes as i32)
    let alltypes = i32::from_ne_bytes(data[0..4].try_into().unwrap());
    if alltypes != 1 {
        std::process::exit(1);
    }

    // Check u.ni.type (first 4 bytes as i32)
    let ni_type = i32::from_ne_bytes(data[0..4].try_into().unwrap());
    if ni_type != 1 {
        std::process::exit(1);
    }

    // Check u.nf.type (first 4 bytes as i32)
    let nf_type = i32::from_ne_bytes(data[0..4].try_into().unwrap());
    if nf_type != 1 {
        std::process::exit(2);
    }

    // Check u.nf.doublenode (bytes 4..12 as f64)
    let nf_doublenode = f64::from_ne_bytes(data[4..12].try_into().unwrap());
    if nf_doublenode != 3.14 {
        std::process::exit(3);
    }

    std::process::exit(0);
}