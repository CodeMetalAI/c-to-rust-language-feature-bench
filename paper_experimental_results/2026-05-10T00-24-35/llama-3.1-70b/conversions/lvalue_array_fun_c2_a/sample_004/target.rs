fn main() {
    const CX: i32 = 9;
    let ax: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(11);

    let a = [1, 2, 3];

    let pa = &a;
    if pa[0]!= 1 {
        std::process::exit(1);
    }
    if pa[1]!= 2 {
        std::process::exit(2);
    }
    if pa[2]!= 3 {
        std::process::exit(3);
    }

    if type_id(&a)!= type_id(&[i32; 3]) {
        std::process::exit(4);
    }
    if type_id(&pa[0])!= type_id(&i32) {
        std::process::exit(5);
    }

    if type_id(&CX)!= type_id(&i32) {
        std::process::exit(6);
    }
    if CX!= 9 {
        std::process::exit(8);
    }

    if type_id(&ax)!= type_id(&std::sync::atomic::AtomicI32) {
        std::process::exit(9);
    }
    if type_id(&ax)!= type_id(&std::sync::atomic::AtomicI32) {
        std::process::exit(10);
    }
    if ax.load(std::sync::atomic::Ordering::SeqCst)!= 11 {
        std::process::exit(12);
    }

    fn id(v: i32) -> i32 {
        v + 1
    }

    let fp: fn(i32) -> i32 = id;
    if type_id(&fp)!= type_id(&fn(i32) -> i32) {
        std::process::exit(13);
    }
    if fp(4)!= 5 {
        std::process::exit(14);
    }
    if id(4)!= 5 {
        std::process::exit(15);
    }

    if std::mem::size_of_val(&a)!= 3 * std::mem::size_of::<i32>() {
        std::process::exit(16);
    }
    if std::mem::align_of::<[i32; 3]>()!= std::mem::align_of::<i32>() {
        std::process::exit(17);
    }

    std::process::exit(0);
}

fn type_id<T>(_x: &T) -> usize {
    std::mem::discriminant(&std::any::TypeId::of::<T>())
}