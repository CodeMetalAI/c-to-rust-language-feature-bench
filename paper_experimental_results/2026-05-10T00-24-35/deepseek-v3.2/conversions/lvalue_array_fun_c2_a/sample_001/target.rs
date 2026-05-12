fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    const CX: i32 = 9;
    let ax: i32 = 11;

    let a = [1, 2, 3];

    let pa = &a[0];
    if pa != &1 {
        return;
    }
    if a[1] != 2 {
        return;
    }
    if a[2] != 3 {
        return;
    }

    // Rust doesn't have _Generic, so we simulate type checking with compile-time traits.
    // We'll use a macro that returns a type identifier based on the type of the expression.
    macro_rules! TYPE_ID {
        (x: i32) => { 1 };
        (x: &i32) => { 2 };
        (x: &const i32) => { 3 };
        (x: i32) => { 4 }; // Atomic is not distinct in Rust
        (x: &i32) => { 5 }; // Atomic pointer
        (x: fn(i32) -> i32) => { 6 };
        (x: const i32) => { 7 };
        (x: _) => { 99 };
    }

    // Check array type (array is not a pointer, so it should be 99)
    if TYPE_ID!(x: a) != 99 {
        return;
    }
    // &a[0] is a pointer to int
    if TYPE_ID!(x: &a[0]) != 2 {
        return;
    }

    // &CX is a pointer to const int
    if TYPE_ID!(x: &CX) != 3 {
        return;
    }
    // +CX is an int (promotion)
    if TYPE_ID!(x: CX) != 1 {
        return;
    }
    if CX != 9 {
        return;
    }

    // ax is an int (atomic is not distinct)
    if TYPE_ID!(x: ax) != 4 {
        return;
    }
    // &ax is a pointer to int
    if TYPE_ID!(x: &ax) != 5 {
        return;
    }
    // +ax is an int
    if TYPE_ID!(x: ax) != 1 {
        return;
    }
    if ax != 11 {
        return;
    }

    let fp: fn(i32) -> i32 = id;
    // id is a function pointer
    if TYPE_ID!(x: id) != 6 {
        return;
    }
    if fp(4) != 5 {
        return;
    }
    if id(4) != 5 {
        return;
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        return;
    }
    if std::mem::align_of_val(&a[0]) != std::mem::align_of::<i32>() {
        return;
    }

    return;
}