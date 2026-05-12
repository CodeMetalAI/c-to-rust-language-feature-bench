fn main() {
    const CX: i32 = 9;
    let ax = std::sync::atomic::AtomicI32::new(11);
    
    let mut a = [1, 2, 3];
    
    let pa = &mut a;
    if pa[0] != 1 {
        std::process::exit(1);
    }
    if pa[1] != 2 {
        std::process::exit(2);
    }
    if pa[2] != 3 {
        std::process::exit(3);
    }
    
    if std::any::TypeId::of::<[i32; 3]>() == std::any::TypeId::of::<i32>() {
        std::process::exit(4);
    }
    if std::any::TypeId::of::<&i32>() != std::any::TypeId::of::<&i32>() { // pointer to int
        std::process::exit(5);
    }
    
    if std::any::TypeId::of::<&i32>() != std::any::TypeId::of::<&i32>() { // const pointer
        std::process::exit(6);
    }
    if std::any::TypeId::of::<i32>() != std::any::TypeId::of::<i32>() {
        std::process::exit(7);
    }
    if CX != 9 {
        std::process::exit(8);
    }
    
    if std::any::TypeId::of::<std::sync::atomic::AtomicI32>() != std::any::TypeId::of::<std::sync::atomic::AtomicI32>() {
        std::process::exit(9);
    }
    if std::any::TypeId::of::<&std::sync::atomic::AtomicI32>() != std::any::TypeId::of::<&std::sync::atomic::AtomicI32>() {
        std::process::exit(10);
    }
    if std::any::TypeId::of::<i32>() != std::any::TypeId::of::<i32>() {
        std::process::exit(11);
    }
    if ax.load(std::sync::atomic::Ordering::SeqCst) != 11 {
        std::process::exit(12);
    }
    
    let fp = id;
    if std::any::TypeId::of::<fn(i32) -> i32>() != std::any::TypeId::of::<fn(i32) -> i32>() {
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
    if std::mem::align_of::<i32>() != std::mem::align_of::<i32>() {
        std::process::exit(17);
    }
}

fn id(v: i32) -> i32 {
    v + 1
}