#[repr(C)]
struct s {
    i: int_anon,
    a: [i32; 4],
}

#[repr(C)]
struct int_anon {
    i: i32,
}

fn main() {
    let layout = std::mem::size_of::<s>();
    let offset_i = unsafe { std::mem::offset::<s, int_anon>(0) as usize };
    let offset_a = unsafe { std::mem::offset::<s, [i32; 4]>(0) as usize };

    if offset_i != 0 {
        return 1;
    }

    if offset_a != layout {
        return 2;
    }

    let mut p = Vec::with_capacity(layout + 4 * std::mem::size_of::<i32>());
    unsafe {
        let ptr = p.as_mut_ptr();
        (*ptr).i = 7;
        (*ptr).a[0] = 11;
        (*ptr).a[3] = 22;

        if (*ptr).i != 7 || (*ptr).a[0] != 11 || (*ptr).a[3] != 22 {
            return 4;
        }
    }

    return 0;
}