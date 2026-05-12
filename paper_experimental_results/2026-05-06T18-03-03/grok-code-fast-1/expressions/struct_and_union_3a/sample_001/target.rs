fn main() {
    const SIZE: usize = 12;
    let mut bytes = [0u8; SIZE];

    // Set nf.type (first 4 bytes)
    bytes[0..4].copy_from_slice(&1i32.to_le_bytes());

    // Set nf.doublenode (next 8 bytes)
    bytes[4..12].copy_from_slice(&3.14f64.to_le_bytes());

    // Check n.alltypes (first 4 bytes)
    let alltypes = i32::from_le_bytes(bytes[0..4].try_into().unwrap());
    if alltypes != 1 {
        std::process::exit(1);
    }

    // Check ni.type (first 4 bytes)
    let ni_type = i32::from_le_bytes(bytes[0..4].try_into().unwrap());
    if ni_type != 1 {
        std::process::exit(1);
    }

    // Check nf.type (first 4 bytes)
    let nf_type = i32::from_le_bytes(bytes[0..4].try_into().unwrap());
    if nf_type != 1 {
        std::process::exit(2);
    }

    // Check nf.doublenode (bytes 4..12)
    let doublenode = f64::from_le_bytes(bytes[4..12].try_into().unwrap());
    if doublenode != 3.14 {
        std::process::exit(3);
    }

    std::process::exit(0);
}