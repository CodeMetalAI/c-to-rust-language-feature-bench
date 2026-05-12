// structure_and_union_spec_1.rs

struct V {
    union_members: enum UnionMember {
        inner1 {
            i: i32,
            j: i32,
        },
        inner2 {
            k: i64,
            l: i64,
        }
    },
    m: i32,
}

impl V {
    fn new() -> Self {
        V {
            union_members: UnionMember::inner1 { i: 0, j: 0 },
            m: 0,
        }
    }
}

enum UnionMember {
    inner1 { i: i32, j: i32 },
    inner2 { k: i64, l: i64 },
}

fn main() {
    let v1 = V::new();
    v1.union_members = UnionMember::inner1 { i: 2, j: 0 };
    v1.union_members = UnionMember::inner2 { k: 5, l: 0 };

    if v1.union_members == UnionMember::inner1 { i: 2, j: 0 } {
        return 1;
    }

    if v1.union_members == UnionMember::inner2 { k: 5, l: 0 } {
        return 1;
    }

    return 0;
}