fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    let cx: i32 = 9;
    let mut ax: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(11);

    let a: [i32; 3] = [1, 2, 3];

    let pa: &[i32] = &a;
    if pa[0] != 1 {
        std::process::exit(1);
    }
    if pa[1] != 2 {
        std::process::exit(2);
    }
    if pa[2] != 3 {
        std::process::exit(3);
    }

    if !std::is_same::<<[i32; 3] as *const _>::Target, *const _>().type_id() == std::any::TypeId::of::<*const _>() {
        std::process::exit(4);
    }
    if !std::is_same::<&i32 as *const _>::Target, *const _>().type_id() == std::any::TypeId::of::<&i32>() {
        std::process::exit(5);
    }

    if !std::is_same::<&i32 as *const _>::Target, *const _>().type_id() == std::any::TypeId::of::<&i32>() {
        std::process::exit(6);
    }
    if cx != 9 {
        std::process::exit(8);
    }

    let ax_ref = &ax;
    if !std::is_same::<std::sync::atomic::AtomicI32 as *const _>::Target, *const _>().type_id() == std::any::TypeId::of::<&std::sync::atomic::AtomicI32>() {
        std::process::exit(9);
    }
    if !std::is_same::<&std::sync::atomic::AtomicI32 as *const _>::Target, *const _>().type_id() == std::any::TypeId::of::<&std::sync::atomic::AtomicI32>() {
        std::process::exit(10);
    }
    if ax.load(std::sync::atomic::Ordering::SeqCst) != 11 {
        std::process::exit(12);
    }

    let fp = id;
    if fp(4) != 5 {
        std::process::exit(14);
    }
    if id(4) != 5 {
        std::process::exit(15);
    }

    let size = std::mem::size_of_val(&a);
    if size != 3 * std::mem::size_of::<i32>() {
        std::process::exit(16);
    }
    let align = std::mem::align_of_val(&a[0]);
    if align != std::mem::align_of::<i32>() {
        std::process::exit(17);
    }

    std::process::exit(0);
}