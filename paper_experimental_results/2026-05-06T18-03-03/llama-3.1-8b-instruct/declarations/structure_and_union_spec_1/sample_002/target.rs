fn main() {
    #[derive(Debug)]
    struct V {
        union: Union,
        m: i32,
    }

    #[derive(Debug)]
    enum Union {
        Struct1 { i: i32, j: i32 },
        Struct2 { k: i64, l: i64 },
    }

    let mut v1 = V {
        union: Union::Struct1 { i: 2, j: 0 },
        m: 0,
    };
    v1.union = Union::Struct2 { k: 5, l: 0 };

    if v1.union == Union::Struct1 { i: 2, j: 0 } {
        return 1;
    }

    if v1.union == Union::Struct2 { k: 5, l: 0 } {
        return 1;
    }

    return 0;
}