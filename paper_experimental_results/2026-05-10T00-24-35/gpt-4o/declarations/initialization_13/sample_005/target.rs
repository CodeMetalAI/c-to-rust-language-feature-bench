fn main() {
    #[repr(C)]
    union U {
        any_member: i32,
        u_member: u32,
        bytes: [u8; 4],
    }

    let x = U { any_member: 42 };

    // Unsafe block to access union field
    unsafe {
        if x.any_member != 42 {
            std::process::exit(1);
        }

        let y = U { u_member: 7 };

        if y.u_member != 7 {
            std::process::exit(2);
        }
    }
}