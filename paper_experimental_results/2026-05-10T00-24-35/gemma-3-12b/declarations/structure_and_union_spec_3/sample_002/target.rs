#[repr(C)]
struct s {
    i: int,
    a: [int; 4],
}

fn main() {
    let layout = std::mem::size_of::<s>();
    if layout < 4 {
        return 1;
    }

    let ptr = Box::new(s {
        i: 7,
        a: [11, 0, 0, 22],
    });

    if ptr.i != 7 || ptr.a[0] != 11 || ptr.a[3] != 22 {
        return 4;
    }

    drop(ptr);
    0
}