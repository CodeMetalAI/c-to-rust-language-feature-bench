fn main() {
    let cx = 9;
    let ax = 11;

    let a = [1, 2, 3];

    let pa = &a;
    if pa[0] != 1 {
        std::process::exit(1);
    }
    if pa[1] != 2 {
        std::process::exit(2);
    }
    if pa[2] != 3 {
        std::process::exit(3);
    }

    if type_id(&a) != 99 {
        std::process::exit(4);
    }
    if type_id(&a[0]) != 2 {
        std::process::exit(5);
    }

    if type_id(&cx) != 3 {
        std::process::exit(6);
    }
    if type_id(cx) != 1 {
        std::process::exit(7);
    }
    if cx != 9 {
        std::process::exit(8);
    }

    if type_id(ax) != 4 {
        std::process::exit(9);
    }
    if type_id(&ax) != 5 {
        std::process::exit(10);
    }
    if type_id(ax) != 1 {
        std::process::exit(11);
    }
    if ax != 11 {
        std::process::exit(12);
    }

    let fp = id;
    if type_id(id as fn(i32) -> i32) != 6 {
        std::process::exit(13);
    }
    if fp(4) != 5 {
        std::process::exit(14);
    }
    if id(4) != 5 {
        std::process::exit(15);
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(16);
    }
    if std::mem::align_of_val(&a[0]) != std::mem::align_of::<i32>() {
        std::process::exit(17);
    }
}

fn type_id<T>(_: T) -> i32 {
    use std::any::TypeId;
    let type_id = TypeId::of::<T>();

    if type_id == TypeId::of::<[i32; 3]>() {
        99
    } else if type_id == TypeId::of::<&i32>() {
        2
    } else if type_id == TypeId::of::<&i32>() {
        3
    } else if type_id == TypeId::of::<i32>() {
        1
    } else if type_id == TypeId::of::<i32>() {
        4
    } else if type_id == TypeId::of::<&i32>() {
        5
    } else if type_id == TypeId::of::<fn(i32) -> i32>() {
        6
    } else {
        99
    }
}

fn id(v: i32) -> i32 {
    v + 1
}