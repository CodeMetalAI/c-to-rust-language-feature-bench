fn main() {
    let cx: i32 = 9;
    let ax: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(11);

    let a: [i32; 3] = [1, 2, 3];

    let pa: &[i32] = &a;
    if pa[0]!= 1 {
        std::process::exit(1);
    }
    if pa[1]!= 2 {
        std::process::exit(2);
    }
    if pa[2]!= 3 {
        std::process::exit(3);
    }

    if!std::any::type_name::<[i32; 3]>().starts_with("i32") {
        std::process::exit(4);
    }
    if!std::any::type_name::<&i32>().starts_with("&i32") {
        std::process::exit(5);
    }

    if!std::any::type_name::<&i32>().starts_with("&i32") {
        std::process::exit(6);
    }
    if cx!= 9 {
        std::process::exit(8);
    }

    if!std::any::type_name::<std::sync::atomic::AtomicI32>().starts_with("AtomicI32") {
        std::process::exit(9);
    }
    if!std::any::type_name::<&std::sync::atomic::AtomicI32>().starts_with("&AtomicI32") {
        std::process::exit(10);
    }
    if ax.load(std::sync::atomic::Ordering::SeqCst)!= 11 {
        std::process::exit(12);
    }

    let id = |x: i32| x + 1;
    if id(4)!= 5 {
        std::process::exit(14);
    }
    if id(4)!= 5 {
        std::process::exit(15);
    }

    if std::mem::size_of_val(&a)!= 3 * std::mem::size_of::<i32>() {
        std::process::exit(16);
    }
    if std::mem::align_of::<i32>()!= std::mem::align_of_val(&a[0]) {
        std::process::exit(17);
    }

    std::process::exit(0);
}