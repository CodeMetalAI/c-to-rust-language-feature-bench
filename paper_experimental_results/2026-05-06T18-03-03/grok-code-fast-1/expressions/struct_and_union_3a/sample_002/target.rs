fn main() -> i32 {
    let mut u = [0u8; 16];

    // u.nf.type = 1
    u[0..4].copy_from_slice(&1i32.to_le_bytes());

    // u.nf.doublenode = 3.14
    u[8..16].copy_from_slice(&3.14f64.to_le_bytes());

    // check u.n.alltypes == 1
    let alltypes = i32::from_le_bytes(u[0..4].try_into().unwrap());
    if alltypes != 1 {
        return 1;
    }

    // check u.ni.type == 1
    let ni_type = i32::from_le_bytes(u[0..4].try_into().unwrap());
    if ni_type != 1 {
        return 1;
    }

    // check u.nf.type == 1
    let nf_type = i32::from_le_bytes(u[0..4].try_into().unwrap());
    if nf_type != 1 {
        return 2;
    }

    // check u.nf.doublenode == 3.14
    let doublenode = f64::from_le_bytes(u[8..16].try_into().unwrap());
    if (doublenode - 3.14).abs() > f64::EPSILON {
        return 3;
    }

    0
}