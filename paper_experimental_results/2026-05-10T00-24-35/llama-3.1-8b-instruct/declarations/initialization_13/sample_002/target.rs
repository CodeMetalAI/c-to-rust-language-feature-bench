#[derive(Debug)]
enum UnionMember {
    AnyMember(i32),
    UMember(u32),
    Bytes([u8; 4]),
}

fn main() {
    let x = UnionMember::AnyMember(42);

    if x.0!= 42 {
        return 1;
    }

    {
        let y = UnionMember::UMember(7);
        if y.0!= 7 {
            return 2;
        }
    }

    return 0;
}