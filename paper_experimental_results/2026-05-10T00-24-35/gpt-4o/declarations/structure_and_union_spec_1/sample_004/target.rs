struct V {
    inner: Inner,
    m: i32,
}

union Inner {
    ij: IJ,
    w: W,
}

#[repr(C)]
struct IJ {
    i: i32,
    j: i32,
}

#[repr(C)]
struct W {
    k: i64,
    l: i64,
}

fn main() {
    let mut v1 = V {
        inner: Inner {
            ij: IJ { i: 0, j: 0 },
        },
        m: 0,
    };

    unsafe {
        v1.inner.ij.i = 2;
        v1.inner.w.k = 5;

        if v1.inner.ij.i != 2 {
            std::process::exit(1);
        }

        if v1.inner.w.k != 5 {
            std::process::exit(1);
        }
    }
}