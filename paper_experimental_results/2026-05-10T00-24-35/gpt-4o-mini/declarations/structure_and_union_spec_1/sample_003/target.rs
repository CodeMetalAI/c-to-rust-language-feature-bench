struct V {
    // Union is represented using an enum
    data: Data,
    m: i32,
}

enum Data {
    Struct1 { i: i32, j: i32 },
    Struct2 { k: i64, l: i64 },
}

fn main() {
    let mut v1 = V {
        data: Data::Struct1 { i: 2, j: 0 },
        m: 0,
    };

    if let Data::Struct1 { ref mut i, .. } = v1.data {
        *i = 2;
    }

    v1.data = Data::Struct2 { k: 5, l: 0 };

    if let Data::Struct1 { i, .. } = v1.data {
        if i != 2 {
            std::process::exit(1);
        }
    }

    if let Data::Struct2 { k, .. } = v1.data {
        if k != 5 {
            std::process::exit(1);
        }
    }

    std::process::exit(0);
}