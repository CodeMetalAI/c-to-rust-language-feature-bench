#[repr(C)]
enum TypeId {
    Int,
    IntPtr,
    ConstIntPtr,
    AtomicInt,
    AtomicIntPtr,
    FuncIntInt,
    ConstInt,
    Default,
}

impl TypeId {
    fn to_usize(x: &dyn std::any::Any) -> usize {
        match x {
            &_ | &_: &i32 => 1,
            &_: &i32 => 2,
            &_: &i32 => 3,
            &_: &std::sync::atomic::AtomicI32 => 4,
            &_: &std::sync::atomic::AtomicI32 => 5,
            &_: &fn(i32) -> i32 => 6,
            &_: &i32 => 7,
            _ => 99,
        }
    }
}

fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    const cx: i32 = 9;
    let ax = std::sync::atomic::AtomicI32::new(11);

    let a = [1, 2, 3];

    let pa = a.as_ptr();
    if pa.add(0).read() != 1 {
        std::process::exit(1);
    }
    if pa.add(1).read() != 2 {
        std::process::exit(2);
    }
    if pa.add(2).read() != 3 {
        std::process::exit(3);
    }

    let type_id_a = TypeId::to_usize(&a);
    if type_id_a != 99 {
        std::process::exit(4);
    }
    let type_id_pa0 = TypeId::to_usize(&pa.add(0));
    if type_id_pa0 != 2 {
        std::process::exit(5);
    }

    let type_id_cx = TypeId::to_usize(&cx);
    if type_id_cx != 3 {
        std::process::exit(6);
    }
    let type_id_cx_plus = TypeId::to_usize(&cx);
    if type_id_cx_plus != 1 {
        std::process::exit(7);
    }
    if cx != 9 {
        std::process::exit(8);
    }

    let type_id_ax = TypeId::to_usize(&ax);
    if type_id_ax != 4 {
        std::process::exit(9);
    }
    let type_id_ax_ptr = TypeId::to_usize(&ax.as_ptr());
    if type_id_ax_ptr != 5 {
        std::process::exit(10);
    }
    let type_id_ax_plus = TypeId::to_usize(&ax);
    if type_id_ax_plus != 1 {
        std::process::exit(11);
    }
    if ax.load(std::sync::atomic::Ordering::Relaxed) != 11 {
        std::process::exit(12);
    }

    let fp: fn(i32) -> i32 = id;
    let type_id_id = TypeId::to_usize(&id);
    if type_id_id != 6 {
        std::process::exit(13);
    }
    if fp(4) != 5 {
        std::process::exit(14);
    }
    if id(4) != 5 {
        std::process::exit(15);
    }

    if std::mem::size_of::<[i32; 3]>() != 3 * std::mem::size_of::<i32>() {
        std::process::exit(16);
    }
    if std::mem::align_of::<i32>() != std::mem::align_of::<[i32; 3]>(){
        std::process::exit(17);
    }

    std::process::exit(0);
}