use std::process::exit;

#[derive(Default)]
struct U {
    any_member: i32,
    u_member: u32,
    bytes: [u8; 4],
}

fn main() {
    let x = U {
        any_member: 42,
        ..Default::default()
    };

    if x.any_member != 42 {
        exit(1);
    }

    {
        let y = U {
            u_member: 7,
            ..Default::default()
        };
        if y.u_member != 7 {
            exit(2);
        }
    }
}