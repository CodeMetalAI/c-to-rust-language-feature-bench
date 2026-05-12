use std::any::TypeId;
use std::mem::{align_of, align_of_val, size_of};
use std::process::exit;
use std::sync::atomic::{AtomicI32, Ordering};

fn type_id_of<T: 'static>() -> i32 {
    let tid = TypeId::of::<T>();
    if tid == TypeId::of::<i32>() {
        1
    } else if tid == TypeId::of::<&mut i32>() {
        2
    } else if tid == TypeId::of::<&i32>() {
        3
    } else if tid == TypeId::of::<AtomicI32>() {
        4
    } else if tid == TypeId::of::<&AtomicI32>() {
        5
    } else if tid == TypeId::of::<fn(i32) -> i32>() {
        6
    } else {
        99
    }
}

fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    let cx: i32 = 9;
    let ax = AtomicI32::new(11);

    let mut a = [1i32, 2, 3];

    {
        let pa = &a;
        if pa[0] != 1 {
            exit(1);
        }
        if pa[1] != 2 {
            exit(2);
        }
        if pa[2] != 3 {
            exit(3);
        }
    }

    if type_id_of::<[i32; 3]>() != 99 {
        exit(4);
    }

    {
        let _pa0: &mut i32 = &mut a[0];
        if type_id_of::<&mut i32>() != 2 {
            exit(5);
        }
    }

    if type_id_of::<&i32>() != 3 {
        exit(6);
    }
    if type_id_of::<i32>() != 1 {
        exit(7);
    }
    if cx != 9 {
        exit(8);
    }

    if type_id_of::<AtomicI32>() != 4 {
        exit(9);
    }
    if type_id_of::<&AtomicI32>() != 5 {
        exit(10);
    }
    if type_id_of::<i32>() != 1 {
        exit(11);
    }
    if ax.load(Ordering::SeqCst) != 11 {
        exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    if type_id_of::<fn(i32) -> i32>() != 6 {
        exit(13);
    }
    if fp(4) != 5 {
        exit(14);
    }
    if id(4) != 5 {
        exit(15);
    }

    if size_of::<[i32; 3]>() != 3 * size_of::<i32>() {
        exit(16);
    }
    if align_of_val(&a[0]) != align_of::<i32>() {
        exit(17);
    }

    exit(0);
}