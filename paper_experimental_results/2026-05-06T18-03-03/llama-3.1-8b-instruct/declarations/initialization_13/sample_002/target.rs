#[derive(Debug)]
struct U {
    any_member: i32,
    u_member: u32,
    bytes: [u8; 4],
}

fn main() {
    let mut x = U {
        any_member: 42,
       ..Default::default()
    };

    if x.any_member!= 42 {
        return 1;
    }

    {
        let mut y = U {
            u_member: 7,
           ..Default::default()
        };
        if y.u_member!= 7 {
            return 2;
        }
    }

    return 0;
}