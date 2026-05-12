fn main() {
    let cx: i32 = 9;
    let mut ax: i32 = 11;

    let mut a = [1, 2, 3];

    let pa = &a;
    if pa[0]!= 1 {
        println!("Assertion failed: pa[0]!= 1");
        return;
    }
    if pa[1]!= 2 {
        println!("Assertion failed: pa[1]!= 2");
        return;
    }
    if pa[2]!= 3 {
        println!("Assertion failed: pa[2]!= 3");
        return;
    }

    let a_type = type_id(a);
    if a_type!= 99 {
        println!("Assertion failed: type_id(a)!= 99");
        return;
    }
    let ptr_type = type_id(&a[0]);
    if ptr_type!= 2 {
        println!("Assertion failed: type_id(&a[0])!= 2");
        return;
    }

    let cx_type = type_id(cx);
    if cx_type!= 3 {
        println!("Assertion failed: type_id(cx)!= 3");
        return;
    }
    let cx_int_type = type_id(cx as i32);
    if cx_int_type!= 1 {
        println!("Assertion failed: type_id(cx as i32)!= 1");
        return;
    }
    if (cx as i32)!= 9 {
        println!("Assertion failed: (cx as i32)!= 9");
        return;
    }

    let ax_type = type_id(ax);
    if ax_type!= 4 {
        println!("Assertion failed: type_id(ax)!= 4");
        return;
    }
    let ptr_ax_type = type_id(&ax);
    if ptr_ax_type!= 5 {
        println!("Assertion failed: type_id(&ax)!= 5");
        return;
    }
    let ax_int_type = type_id(ax as i32);
    if ax_int_type!= 1 {
        println!("Assertion failed: type_id(ax as i32)!= 1");
        return;
    }
    if (ax as i32)!= 11 {
        println!("Assertion failed: (ax as i32)!= 11");
        return;
    }

    let id = |x: i32| -> i32 { x + 1 };
    let id_type = type_id(id);
    if id_type!= 6 {
        println!("Assertion failed: type_id(id)!= 6");
        return;
    }
    if id(4)!= 5 {
        println!("Assertion failed: id(4)!= 5");
        return;
    }
    if id(4)!= 5 {
        println!("Assertion failed: id(4)!= 5");
        return;
    }

    let size = a.len() * std::mem::size_of::<i32>();
    if size!= 3 * std::mem::size_of::<i32>() {
        println!("Assertion failed: a.len() * std::mem::size_of::<i32>()!= 3 * std::mem::size_of::<i32>()");
        return;
    }
    let alignment = std::mem::align_of::<i32>();
    if std::mem::align_of(a[0])!= alignment {
        println!("Assertion failed: std::mem::align_of(a[0])!= std::mem::align_of::<i32>()");
        return;
    }

    println!("All assertions passed");
}

fn type_id<T>(x: T) -> i32 {
    match std::any::type_id_of::<T>() {
        std::any::TypeId::Int => 1,
        std::any::TypeId::Slice(_) => 2,
        std::any::TypeId::Raw(_) => 3,
        std::any::TypeId::Int(_x) => 4,
        std::any::TypeId::IntSlice(_) => 5,
        std::any::TypeId::Fn(_) => 6,
        std::any::TypeId::Unit => 7,
        _ => 99,
    }
}