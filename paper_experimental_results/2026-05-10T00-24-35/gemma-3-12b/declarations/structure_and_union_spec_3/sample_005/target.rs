#[repr(C)]
struct s {
    i: int_field,
    a: [i32; 4],
}

#[repr(C)]
struct int_field {
    i: i32,
}

fn main() {
    let layout = std::mem::size_of::<s>();
    let offset_i = unsafe { std::mem::offset::<s, int_field>(0) as usize };
    let offset_a = unsafe { std::mem::offset::<s, [i32; 4]>(0) as usize };

    if offset_i != 0 {
        return 1;
    }

    if offset_a != layout {
        return 2;
    }

    let mut p = Box::new(s {
        i: int_field { i: 0 },
        a: [0, 0, 0, 0],
    });

    p.i.i = 7;
    p.a[0] = 11;
    p.a[3] = 22;

    if p.i.i != 7 || p.a[0] != 11 || p.a[3] != 22 {
        return 4;
    }

    // No need to free, Box handles it automatically
    drop(p);


    0
}