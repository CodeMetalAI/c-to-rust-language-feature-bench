// initialization_13.rs
#[derive(Debug, PartialEq)]
struct Union {
    any_member: i32,
    u_member: u32,
    bytes: [u8; 4],
}

fn main() {
    let mut x = Union {
        any_member: 42,
        ..Default::default()
    };

    if x.any_member != 42 {
        std::process::exit(1);
    }

    {
        let mut y = Union {
            u_member: 7,
            ..Default::default()
        };
        if y.u_member != 7 {
            std::process::exit(2);
        }
    }

    std::process::exit(0);
}