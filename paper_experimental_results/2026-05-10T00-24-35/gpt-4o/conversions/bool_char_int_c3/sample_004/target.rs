fn main() {
    let pc: volatile::Volatile<char> = volatile::Volatile::new(-1_i8 as u8 as char);
    let sc: volatile::Volatile<i8> = volatile::Volatile::new(-1_i8);
    let uc: volatile::Volatile<u8> = volatile::Volatile::new(-1_u8);

    let ipc = *pc.get() as i32;
    let isc = *sc.get() as i32;
    let iuc = *uc.get() as i32;

    if !matches!(pc.get().wrapping_add(0 as char) as i32, _ if matches!(TYPE_ID!(0), Type::Int)) {
        std::process::exit(1);
    }
    if !matches!(sc.get().wrapping_add(0) as i32, _ if matches!(TYPE_ID!(0), Type::Int)) {
        std::process::exit(2);
    }
    if !matches!(uc.get().wrapping_add(0) as i32, _ if matches!(TYPE_ID!(0), Type::Int)) {
        std::process::exit(3);
    }

    if isc != -1 {
        std::process::exit(4);
    }

    if -1_i8 < 0 {
        if ipc != -1 {
            std::process::exit(5);
        }
        if (iuc as u32) != 255 {
            std::process::exit(6);
        }
    } else {
        if (ipc as u32) != 255 {
            std::process::exit(7);
        }
        if (iuc as u32) != 255 {
            std::process::exit(8);
        }
    }
}

#[derive(Debug, PartialEq)]
enum Type {
    Int,
    UnsignedInt,
    Default,
}

macro_rules! TYPE_ID {
    ($x:expr) => {
        match $x {
            _ if $x.is::<i32>() => Type::Int,
            _ if $x.is::<u32>() => Type::UnsignedInt,
            _ => Type::Default,
        }
    };
}

trait TypeId {
    fn is<T: 'static>(&self) -> bool;
}

impl<T: 'static> TypeId for T {
    fn is<U: 'static>(&self) -> bool {
        std::any::TypeId::of::<T>() == std::any::TypeId::of::<U>()
    }
}

mod volatile {
    use std::cell::Cell;

    pub struct Volatile<T: Copy> {
        inner: Cell<T>,
    }

    impl<T: Copy> Volatile<T> {
        pub fn new(value: T) -> Volatile<T> {
            Volatile {
                inner: Cell::new(value),
            }
        }

        pub fn get(&self) -> T {
            self.inner.get()
        }

        pub fn set(&self, value: T) {
            self.inner.set(value);
        }
    }
}