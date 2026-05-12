#[repr(C)]
union U {
    any_member: i32,
    u_member: u32,
    bytes: [u8; 4],
}

fn main() -> i32 {
    unsafe {
        let mut x = U { any_member: 42 };
        if x.any_member != 42 {
            return 1;
        }

        {
            let mut y = U { u_member: 7u32 };
            if y.u_member != 7u32 {
                return 2;
            }
        }
    }
    0
}