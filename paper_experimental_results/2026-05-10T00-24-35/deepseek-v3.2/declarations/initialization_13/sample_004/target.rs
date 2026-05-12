#[repr(C)]
union U {
    any_member: i32,
    u_member: u32,
    bytes: [u8; 4],
}

fn main() {
    let x = U { any_member: 42 };

    unsafe {
        if x.any_member != 42 {
            return;
        }
    }

    let y = U { u_member: 7 };

    unsafe {
        if y.u_member != 7 {
            return;
        }
    }
}