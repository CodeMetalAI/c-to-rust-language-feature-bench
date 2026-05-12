#[repr(C)]
struct St {
    a: i32,
    b: i32,
}

#[repr(C)]
union Un {
    i: i32,
    d: f64,
}

fn main() -> i32 {
    let x = 0;
    let px1 = &x as *const i32 as usize;
    let px2 = &x as *const i32 as usize;
    if px1 != px2 {
        return 1;
    }

    let arr: [i32; 3] = [1, 2, 3];
    let arr_ptr = arr.as_ptr() as usize;
    let elem_size = std::mem::size_of::<i32>();
    let p_end1 = arr_ptr + 3 * elem_size;
    let p_end2 = arr_ptr + 3 * elem_size;
    if p_end1 != p_end2 {
        return 2;
    }

    let p0 = arr_ptr;
    let p2 = arr_ptr + 2 * elem_size;
    if !(p2 > p0) {
        return 3;
    }
    if !(p0 < p2) {
        return 4;
    }

    let q_last = arr_ptr + 2 * elem_size;
    let q1 = q_last + elem_size;
    if !(q1 > p0) {
        return 5;
    }

    let s = St { a: 0, b: 0 };
    let sa = &s.a as *const i32 as usize;
    let sb = &s.b as *const i32 as usize;
    if !(sb > sa) {
        return 6;
    }

    let u = Un { i: 0 };
    let u_addr = &u as *const Un as usize;
    let ui = u_addr;
    let ud = u_addr;
    if ui != ud {
        return 7;
    }

    0
}