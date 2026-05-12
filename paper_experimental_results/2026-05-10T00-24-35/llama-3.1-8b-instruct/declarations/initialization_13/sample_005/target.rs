#[derive(Debug, PartialEq)]
union U {
    any_member: i32,
    u_member: u32,
    bytes: [u8; 4],
}

fn main() {
    let x = U { any_member: 42 };

    if x.any_member!= 42 {
        return 1;
    }

    {
        let y = U { u_member: 7 };
        if y.u_member!= 7 {
            return 2;
        }
    }

    return 0;
}