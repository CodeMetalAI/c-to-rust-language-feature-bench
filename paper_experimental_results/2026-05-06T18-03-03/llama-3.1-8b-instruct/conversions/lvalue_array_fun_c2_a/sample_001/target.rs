#![allow(unused_imports)]

use std::sync::atomic::{AtomicIsize, Ordering};

fn id(x: isize) -> isize {
    x + 1
}

fn main() {
    let cx: isize = 9;
    let ax = AtomicIsize::new(11);

    let a = [1, 2, 3];

    let pa = &a;
    if pa[0] != 1 {
        println!("pa[0] != 1");
        std::process::exit(1);
    }
    if pa[1] != 2 {
        println!("pa[1] != 2");
        std::process::exit(2);
    }
    if pa[2] != 3 {
        println!("pa[2] != 3");
        std::process::exit(3);
    }

    if std::any::type_id(&a) != std::any::type_id(&[1, 2, 3]) {
        println!("TYPE_ID(a) != 99");
        std::process::exit(4);
    }
    if std::any::type_id(&a[0]) != std::any::type_id(&1) {
        println!("TYPE_ID(&a[0]) != 2");
        std::process::exit(5);
    }

    if std::any::type_id(&cx) != std::any::type_id(&9) {
        println!("TYPE_ID(&cx) != 3");
        std::process::exit(6);
    }
    if std::any::type_id(cx) != std::any::type_id(9) {
        println!("TYPE_ID(cx) != 1");
        std::process::exit(7);
    }
    if cx != 9 {
        println!("cx != 9");
        std::process::exit(8);
    }

    if std::any::type_id(ax) != std::any::type_id(AtomicIsize::new(11)) {
        println!("TYPE_ID(ax) != 4");
        std::process::exit(9);
    }
    if std::any::type_id(&ax) != std::any::type_id(&AtomicIsize::new(11)) {
        println!("TYPE_ID(&ax) != 5");
        std::process::exit(10);
    }
    if std::any::type_id(ax) != std::any::type_id(AtomicIsize::new(11)) {
        println!("TYPE_ID(ax) != 1");
        std::process::exit(11);
    }
    if ax.load(Ordering::SeqCst) != 11 {
        println!("ax != 11");
        std::process::exit(12);
    }

    let fp = id;
    if std::any::type_id(fp) != std::any::type_id(id) {
        println!("TYPE_ID(id) != 6");
        std::process::exit(13);
    }
    if fp(4) != 5 {
        println!("fp(4) != 5");
        std::process::exit(14);
    }
    if id(4) != 5 {
        println!("id(4) != 5");
        std::process::exit(15);
    }

    if std::mem::size_of::<[isize; 3]>() != 3 * std::mem::size_of::<isize>() {
        println!("sizeof a != 3 * sizeof(int)");
        std::process::exit(16);
    }
    if std::mem::align_of::<isize>() != std::mem::align_of::<isize>() {
        println!("_Alignof(a[0]) != _Alignof(int)");
        std::process::exit(17);
    }

    println!("All tests passed");
    std::process::exit(0);
}