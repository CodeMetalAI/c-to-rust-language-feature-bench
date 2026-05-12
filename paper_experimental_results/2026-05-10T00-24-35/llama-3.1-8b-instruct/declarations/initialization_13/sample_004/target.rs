union U {
    i32 any_member: i32;
    u32 u_member: u32;
    [4] u8 bytes: [4];
}

fn main() -> i32 {
    let mut x: U = U { any_member: 42 };

    if x.any_member != 42 {
        1
    } else {
        {
            let mut y: U = U { u_member: 7 };
            if y.u_member != 7 {
                2
            } else {
                0
            }
        }
    }
}