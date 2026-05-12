union U {
    int any_member: u32,
    u32 u_member,
    [4] u8 bytes,
}

fn main() {
    let mut x = U { any_member: 42 };

    if x.any_member!= 42 {
        return 1;
    }

    {
        let mut y = U { u_member: 7 };
        if y.u_member!= 7 {
            return 2;
        }
    }

    return 0;
}