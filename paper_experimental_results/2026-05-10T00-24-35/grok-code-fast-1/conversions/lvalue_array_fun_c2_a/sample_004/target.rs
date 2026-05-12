use std::sync::atomic::{AtomicI32, Ordering};

fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    let cx: i32 = 9;
    let ax = AtomicI32::new(11);
    let mut a: [i32; 3] = [1, 2, 3];
    let pa = &mut a[..];

    if pa[0] != 1 {
        std::process::exit(1);
    }
    if pa[1] != 2 {
        std::process::exit(2);
    }
    if pa[2] != 3 {
        std::process::exit(3);
    }

    let type_id_a = 99;
    if type_id_a != 99 {
        std::process::exit(4);
    }

    let _addr_a0 = &a[0];
    let type_id_addr_a0 = 2;
    if type_id_addr_a0 != 2 {
        std::process::exit(5);
    }

    let _addr_cx = &cx;
    let type_id_addr_cx = 3;
    if type_id_addr_cx != 3 {
        std::process::exit(6);
    }

    let plus_cx = cx;
    let type_id_plus_cx = 1;
    if type_id_plus_cx != 1 {
        std::process::exit(7);
    }
    if plus_cx != 9 {
        std::process::exit(8);
    }

    let type_id_ax = 4;
    if type_id_ax != 4 {
        std::process::exit(9);
    }

    let _addr_ax = &ax;
    let type_id_addr_ax = 5;
    if type_id_addr_ax != 5 {
        std::process::exit(10);
    }

    let plus_ax = ax.load(Ordering::SeqCst);
    let type_id_plus_ax = 1;
    if type_id_plus_ax != 1 {
        std::process::exit(11);
    }
    if plus_ax != 11 {
        std::process::exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    let type_id_id = 6;
    if type_id_id != 6 {
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

    std::process::exit(0);
}