fn main() {
    struct U {
        any_member: i32,
        u_member: u32,
        bytes: [u8; 4],
    }

    let x = U {
        any_member: 42,
        u_member: 0,
        bytes: [0; 4],
    };

    if x.any_member != 42 {
        std::process::exit(1);
    }

    {
        let y = U {
            any_member: 0,
            u_member: 7,
            bytes: [0; 4],
        };
        if y.u_member != 7 {
            std::process::exit(2);
        }
    }

    std::process::exit(0);
}