#[derive(Debug)]
#[repr(C)]
union U {
    any_member: i32,
    u_member: u32,
    bytes: [u8; 4],
}

fn main() {
    let x = U { any_member: 42 };

    if x.any_member != 42 {
        std::process::exit(1);
    }

    {
        let y = U { u_member: 7u32 };
        if y.u_member != 7 {
            std::process::exit(2);
        }
    }

    std::process::exit(0);
}