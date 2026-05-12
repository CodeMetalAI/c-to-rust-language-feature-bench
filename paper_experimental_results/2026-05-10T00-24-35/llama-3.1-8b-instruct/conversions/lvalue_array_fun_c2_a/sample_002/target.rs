fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    let cx = 9;
    let mut ax = 11;

    let a = [1, 2, 3];

    let pa = &a[0];
    if pa != &1 {
        println!("pa != 1");
        return;
    }
    if pa == &2 {
        println!("pa == 2");
        return;
    }
    if pa != &3 {
        println!("pa != 3");
        return;
    }

    match a {
        [..] => {},
        _ => panic!("TYPE_ID(a) != 99"),
    }
    match &a[0] {
        2 => {},
        _ => panic!("TYPE_ID(&a[0]) != 2"),
    }

    match cx {
        9 => {},
        _ => panic!("TYPE_ID(&cx) != 3"),
    }
    match +cx {
        9 => {},
        _ => panic!("(+cx) != 9"),
    }

    match ax {
        11 => {},
        _ => panic!("TYPE_ID(ax) != 4"),
    }
    match &ax {
        11 => {},
        _ => panic!("TYPE_ID(&ax) != 5"),
    }
    match +ax {
        11 => {},
        _ => panic!("(+ax) != 11"),
    }

    let fp = id;
    match fp {
        id => {},
        _ => panic!("TYPE_ID(id) != 6"),
    }
    match fp(4) {
        5 => {},
        _ => panic!("fp(4) != 5"),
    }
    match id(4) {
        5 => {},
        _ => panic!("id(4) != 5"),
    }

    assert_eq!(a.len(), 3);
    assert_eq!(std::mem::size_of::<i32>(), 4);
    assert_eq!(std::mem::align_of::<i32>(), 4);
}