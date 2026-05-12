use std::convert::TryInto;
use std::mem;

#[repr(C)]
struct N {
    alltypes: i32,
}

#[repr(C)]
struct Ni {
    type_: i32,
    intnode: i32,
}

#[repr(C)]
struct Nf {
    type_: i32,
    doublenode: f64,
}

fn main() {
    let offset_nf_doublenode = mem::offset_of!(Nf, doublenode);
    let size_union = mem::size_of::<Nf>();
    let mut mem: Vec<u8> = vec![0; size_union];

    // u.nf.type = 1;
    mem[0..4].copy_from_slice(&1i32.to_le_bytes());

    // u.nf.doublenode = 3.14;
    mem[offset_nf_doublenode..offset_nf_doublenode + 8].copy_from_slice(&3.14f64.to_le_bytes());

    // if (u.n.alltypes != 1) return 1;
    let alltypes = i32::from_le_bytes(mem[0..4].try_into().unwrap());
    if alltypes != 1 {
        std::process::exit(1);
    }

    // if (u.ni.type != 1) return 1;
    let type_ni = i32::from_le_bytes(mem[0..4].try_into().unwrap());
    if type_ni != 1 {
        std::process::exit(1);
    }

    // if (u.nf.type != 1) return 2;
    let type_nf = i32::from_le_bytes(mem[0..4].try_into().unwrap());
    if type_nf != 1 {
        std::process::exit(2);
    }

    // if (u.nf.doublenode != 3.14) return 3;
    let doublenode = f64::from_le_bytes(mem[offset_nf_doublenode..offset_nf_doublenode + 8].try_into().unwrap());
    if doublenode != 3.14 {
        std::process::exit(3);
    }

    std::process::exit(0);
}