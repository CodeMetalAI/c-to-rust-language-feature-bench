use std::marker::PhantomData;

#[derive(Copy, Clone)]
struct Pointer<T> {
    is_null: bool,
    _phantom: PhantomData<T>,
}

impl<T> PartialEq for Pointer<T> {
    fn eq(&self, other: &Self) -> bool {
        self.is_null && other.is_null
    }
}

fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = 1;
    let px = Pointer::<i32> {
        is_null: false,
        _phantom: PhantomData,
    };
    let pf = Pointer::<fn(i32) -> i32> {
        is_null: false,
        _phantom: PhantomData,
    };

    let n1 = Pointer::<i32> {
        is_null: true,
        _phantom: PhantomData,
    };
    let v0 = Pointer::<()> {
        is_null: true,
        _phantom: PhantomData,
    };
    let n2 = Pointer::<i32> {
        is_null: v0.is_null,
        _phantom: PhantomData,
    };

    let nf1 = Pointer::<fn(i32) -> i32> {
        is_null: true,
        _phantom: PhantomData,
    };
    let vf0 = Pointer::<fn(i32) -> i32> {
        is_null: true,
        _phantom: PhantomData,
    };
    let nf2 = Pointer::<fn(i32) -> i32> {
        is_null: vf0.is_null,
        _phantom: PhantomData,
    };

    if !n1.is_null {
        std::process::exit(1);
    }
    if !n2.is_null {
        std::process::exit(2);
    }
    if !v0.is_null {
        std::process::exit(3);
    }

    if n1 == px {
        std::process::exit(4);
    }
    if n2 == px {
        std::process::exit(5);
    }

    if !nf1.is_null {
        std::process::exit(6);
    }
    if !nf2.is_null {
        std::process::exit(7);
    }
    if nf1 == pf {
        std::process::exit(8);
    }
    if nf2 == pf {
        std::process::exit(9);
    }

    let vx = Pointer::<()> {
        is_null: px.is_null,
        _phantom: PhantomData,
    };
    let vf = Pointer::<()> {
        is_null: pf.is_null,
        _phantom: PhantomData,
    };

    if vx.is_null {
        std::process::exit(10);
    }
    if vf.is_null {
        std::process::exit(11);
    }
    if vx == v0 {
        std::process::exit(12);
    }
    if vf == vf0 {
        std::process::exit(13);
    }

    std::process::exit(0);
}