union U {
    any_member: i32,
    u_member: u32,
    bytes: [u8; 4],
}

fn main() {
    let mut x = U { any_member: 42 };

    if unsafe { x.any_member } != 42 {
        std::process::exit(1);
    }

    {
        let mut y = U { u_member: 7 };
        if unsafe { y.u_member } != 7 {
            std::process::exit(2);
        }
    }
}