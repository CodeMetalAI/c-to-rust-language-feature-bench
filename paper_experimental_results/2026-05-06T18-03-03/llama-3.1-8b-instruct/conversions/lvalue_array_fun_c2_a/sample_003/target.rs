fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    let cx: i32 = 9;
    let mut ax: i32 = 11;

    let a: [i32; 3] = [1, 2, 3];

    let pa: &[i32] = &a;
    if pa[0] != 1 {
        panic!("pa[0] != 1");
    }
    if pa[1] != 2 {
        panic!("pa[1] != 2");
    }
    if pa[2] != 3 {
        panic!("pa[2] != 3");
    }

    let type_id_a: u32 = type_id(a);
    if type_id_a != 99 {
        panic!("type_id(a) != 99");
    }
    let type_id_pa: u32 = type_id(&a[0]);
    if type_id_pa != 2 {
        panic!("type_id(&a[0]) != 2");
    }

    let type_id_cx: u32 = type_id(cx);
    if type_id_cx != 3 {
        panic!("type_id(cx) != 3");
    }
    let type_id_plus_cx: u32 = type_id(cx);
    if type_id_plus_cx != 1 {
        panic!("type_id(+cx) != 1");
    }
    if cx != 9 {
        panic!("+cx != 9");
    }

    let type_id_ax: u32 = type_id(ax);
    if type_id_ax != 4 {
        panic!("type_id(ax) != 4");
    }
    let type_id_pa_ax: u32 = type_id(&ax);
    if type_id_pa_ax != 5 {
        panic!("type_id(&ax) != 5");
    }
    let type_id_plus_ax: u32 = type_id(ax);
    if type_id_plus_ax != 1 {
        panic!("type_id(+ax) != 1");
    }
    if ax != 11 {
        panic!("+ax != 11");
    }

    let fp: fn(i32) -> i32 = id;
    let type_id_fp: u32 = type_id(id);
    if type_id_fp != 6 {
        panic!("type_id(id) != 6");
    }
    if fp(4) != 5 {
        panic!("fp(4) != 5");
    }
    if id(4) != 5 {
        panic!("id(4) != 5");
    }

    if std::mem::size_of::<[i32; 3]>() != 3 * std::mem::size_of::<i32>() {
        panic!("sizeof a != 3 * sizeof(int)");
    }
    if std::mem::align_of::<i32>() != std::mem::align_of::<[i32; 3]>() {
        panic!("alignof(a[0]) != alignof(int)");
    }

    assert_eq!(0, 0);
}

fn type_id<T>(x: T) -> u32 {
    match x {
        _ => 99,
    }
}