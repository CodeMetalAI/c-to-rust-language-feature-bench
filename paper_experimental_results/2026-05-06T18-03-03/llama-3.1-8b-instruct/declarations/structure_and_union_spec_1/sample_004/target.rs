struct V {
    union {
        struct {
            i: i32,
            j: i32,
        },
        struct {
            k: i64,
            l: i64,
        },
    },
    m: i32,
}

fn main() {
    let mut v1 = V {
        union: union! {
            struct {
                i: 2,
                j: 0,
            },
            struct {
                k: 5,
                l: 0,
            },
        },
        m: 0,
    };

    if v1.union.i!= 2 {
        return 1;
    }

    if v1.union.k!= 5 {
        return 1;
    }

    return 0;
}

macro_rules! union {
    ($($struct:ident { $($field:ident: $ty:ty),* }),*) => {
        union {
            $($struct { $($field: $ty),* }),*
        }
    };
}