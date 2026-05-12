fn main() {
    let mut buf = [0u8; 12];

    // u.nf.type = 1;
    buf[0..4].copy_from_slice(&1i32.to_ne_bytes());

    // u.nf.doublenode = 3.14;
    buf[4..12].copy_from_slice(&3.14f64.to_ne_bytes());

    // if (u.n.alltypes != 1) return 1;
    if get_alltypes(&buf) != 1 {
        std::process::exit(1);
    }

    // if (u.ni.type != 1) return 1;
    if get_type(&buf) != 1 {
        std::process::exit(1);
    }

    // if (u.nf.type != 1) return 2;
    if get_nf_type(&buf) != 1 {
        std::process::exit(2);
    }

    // if (u.nf.doublenode != 3.14) return 3;
    if get_doublenode(&buf) != 3.14 {
        std::process::exit(3);
    }

    std::process::exit(0);
}

fn get_alltypes(buf: &[u8; 12]) -> i32 {
    i32::from_ne_bytes(buf[0..4].try_into().unwrap())
}

fn get_type(buf: &[u8; 12]) -> i32 {
    get_alltypes(buf)
}

fn get_nf_type(buf: &[u8; 12]) -> i32 {
    get_alltypes(buf)
}

fn get_doublenode(buf: &[u8; 12]) -> f64 {
    f64::from_ne_bytes(buf[4..12].try_into().unwrap())
}