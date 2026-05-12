fn test_incdec<T>() where T: std::ops::AddAssign + std::ops::SubAssign + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Not + PartialEq + Copy {
    let mut a = std::cell::Cell::new(T::default());
    if a.get()!= T::default() {
        std::process::abort();
    }
    a.set(a.get() + T::default());
    if a.get()!= T::default() {
        std::process::abort();
    }
    a.set(a.get() - T::default());
    if a.get()!= T::default() {
        std::process::abort();
    }
    a.set(T::default() + a.get());
    if a.get()!= T::default() {
        std::process::abort();
    }
    a.set(T::default() - a.get());
    if a.get()!= T::default() {
        std::process::abort();
    }
}

fn test_incdec_ptr<T>() where T: std::ops::AddAssign + std::ops::SubAssign + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Not + PartialEq + Copy {
    let mut a = std::cell::Cell::new(&0 as *const T);
    if a.get()!= &0 as *const T {
        std::process::abort();
    }
    a.set(&*a.get());
    if a.get()!= &0 as *const T {
        std::process::abort();
    }
    a.set(&*a.get());
    if a.get()!= &0 as *const T {
        std::process::abort();
    }
    a.set(&*a.get());
    if a.get()!= &0 as *const T {
        std::process::abort();
    }
    a.set(&*a.get());
    if a.get()!= &0 as *const T {
        std::process::abort();
    }
}

fn test_incdec_all() {
    test_incdec::<bool>();
    test_incdec::<i8>();
    test_incdec::<u8>();
    test_incdec::<i16>();
    test_incdec::<u16>();
    test_incdec::<i32>();
    test_incdec::<u32>();
    test_incdec::<i64>();
    test_incdec::<u64>();
    test_incdec::<f32>();
    test_incdec::<f64>();
    test_incdec_ptr::<i32>();
}

fn main() {
    test_incdec_all();
}