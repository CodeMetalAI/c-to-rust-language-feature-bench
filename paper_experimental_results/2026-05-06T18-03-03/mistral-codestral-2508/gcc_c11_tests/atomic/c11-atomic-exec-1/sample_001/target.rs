use std::process::exit;

fn test_simple_assign<T: Copy + PartialEq + Default>() {
    let mut a: T = Default::default();
    let b: T = T::from(0);

    if a != Default::default() {
        exit(1);
    }
    if b != T::from(0) {
        exit(1);
    }

    a = b;
    if a != T::from(0) {
        exit(1);
    }
}

fn test_simple_assign_arith<T: Copy + PartialEq + Default + From<i32> + From<f64>>() {
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
    test_simple_assign::<T>();
}

fn main() {
    test_simple_assign_arith::<bool>();
    test_simple_assign_arith::<i8>();
    test_simple_assign_arith::<u8>();
    test_simple_assign_arith::<i16>();
    test_simple_assign_arith::<u16>();
    test_simple_assign_arith::<i32>();
    test_simple_assign_arith::<u32>();
    test_simple_assign_arith::<i64>();
    test_simple_assign_arith::<u64>();
    test_simple_assign_arith::<f32>();
    test_simple_assign_arith::<f64>();

    // For complex numbers, we can use tuples (f64, f64) to represent them
    test_simple_assign::<(f64, f64)>();

    // For pointers, we can use Option<&i32> to represent them
    let i = 0;
    test_simple_assign::<Option<&i32>>();
    test_simple_assign::<Option<&i32>>();

    // For structs, we can define a custom struct and implement the necessary traits
    #[derive(Copy, Clone, PartialEq, Default)]
    struct S {
        a: [i16; 1024],
    }

    let mut init = S { a: [0; 1024] };
    let mut copy = S { a: [0; 1024] };

    for j in 0..1024 {
        init.a[j] = j as i16;
    }

    let s1 = init;
    copy = s1;
    if init != copy {
        exit(1);
    }

    let s2 = s1;
    copy = s2;
    if init != copy {
        exit(1);
    }

    copy = s1;
    if init != copy {
        exit(1);
    }

    copy = s2;
    if init != copy {
        exit(1);
    }

    exit(0);
}