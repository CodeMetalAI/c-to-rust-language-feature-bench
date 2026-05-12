fn main() {
    let mut x = 0;
    let px1 = &x;
    let px2 = &x;
    if px1!= px2 {
        panic!("px1!= px2");
    }

    let arr = [1, 2, 3];
    let p_end1 = &arr[3];
    let p_end2 = arr.get_unchecked(3);
    if p_end1!= p_end2 {
        panic!("p_end1!= p_end2");
    }

    let p0 = &arr[0];
    let p2 = &arr[2];
    if!(p2 > p0) {
        panic!("p2 <= p0");
    }
    if!(p0 < p2) {
        panic!("p0 >= p2");
    }

    let q_last = &arr[2];
    let q1 = unsafe { &arr[3] };
    if!(q1 > p0) {
        panic!("q1 <= p0");
    }

    let s = struct_st { a: 0, b: 0 };
    let sa = &s.a as *const _ as *const u8;
    let sb = &s.b as *const _ as *const u8;
    if!(sb > sa) {
        panic!("sb <= sa");
    }

    let u = union_un {};
    let ui = &u.i as *const _ as *const u8;
    let ud = &u.d as *const _ as *const u8;
    if ui!= ud {
        panic!("ui!= ud");
    }
}

struct struct_st {
    a: i32,
    b: i32,
}

union union_un {
    i32: i32,
    d: f64,
}