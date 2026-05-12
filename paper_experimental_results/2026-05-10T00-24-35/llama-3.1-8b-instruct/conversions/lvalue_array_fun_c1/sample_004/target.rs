fn main() {
    let i = 1;
    let p = &i;

    if!sizeof(i) > 0 {
        return 1;
    }
    if!sizeof(*p) > 0 {
        return 2;
    }

    let a = [1, 2];
    (a, a.len());
    (a.len(), 0);

    let ci = 3;
    (ci, 0);

    let s3 = S3 { a: 4 };
    if!sizeof(s3.a) > 0 {
        return 3;
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    (s1, s2);

    {
        let t = 9;
        let bp = t as *const u8;
        bp.read_unaligned::<u8>().unwrap();
        let _ = t;
        if t!= t {
            return 4;
        }
    }

    0
}

#[derive(Debug)]
struct S1 {
    a: i32,
    b: i32,
}

#[derive(Debug)]
struct S2 {
    s: S1,
}

#[derive(Debug)]
struct S3 {
    a: i32,
}

trait SizeOf<T> {
    fn size_of() -> usize;
}

impl<T> SizeOf<T> for T {
    fn size_of() -> usize {
        std::mem::size_of::<T>()
    }
}

trait ReadUnaligned<T> {
    fn read_unaligned(self) -> T;
}

impl<T> ReadUnaligned<T> for *const T {
    fn read_unaligned(self) -> T {
        *self
    }
}

fn sizeof<T>(_: T) -> bool {
    T::size_of() > 0
}