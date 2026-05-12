struct U {
    any_member: i32,
    u_member: u32,
    bytes: [u8; 4],
}

impl U {
    fn new_any_member(any_member: i32) -> Self {
        U {
            any_member,
            u_member: 0,
            bytes: [0; 4],
        }
    }

    fn new_u_member(u_member: u32) -> Self {
        U {
            any_member: 0,
            u_member,
            bytes: [0; 4],
        }
    }
}

fn main() -> i32 {
    let x = U::new_any_member(42);

    if x.any_member != 42 {
        return 1;
    }

    {
        let y = U::new_u_member(7);
        if y.u_member != 7 {
            return 2;
        }
    }

    return 0;
}