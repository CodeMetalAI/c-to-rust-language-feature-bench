#[derive(Clone, Copy)]
enum U {
    AnyMember(i32),
    UMember(u32),
    Bytes([u8; 4]),
}

fn main() -> i32 {
    let x = U::AnyMember(42);
    if let U::AnyMember(val) = x {
        if val != 42 {
            return 1;
        }
    } else {
        return 1;
    }

    {
        let y = U::UMember(7);
        if let U::UMember(val) = y {
            if val != 7 {
                return 2;
            }
        } else {
            return 2;
        }
    }

    0
}