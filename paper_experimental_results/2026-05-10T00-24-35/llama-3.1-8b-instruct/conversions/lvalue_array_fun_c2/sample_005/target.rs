// Define a struct S to match the C code
struct S {
    m: i32,
}

fn main() {
    let x: i32 = 3;
    let cx: i32 = 4;
    let ax: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(5);

    // Compare the size of x with the size of i32
    if std::mem::size_of::<i32>() != std::mem::size_of(&x) {
        panic!("Sizes do not match");
    }

    // Compare the alignment of x with the alignment of i32
    if std::mem::align_of::<i32>() != std::mem::align_of(&x) {
        panic!("Alignments do not match");
    }

    // Compare the value of x with the value at the address of x
    if *(&x) != 3 {
        panic!("Values do not match");
    }

    // Compare the type ID of x with 1 (i32)
    if type_id(&x) != 1 {
        panic!("Type IDs do not match");
    }

    // Compare the type ID of ax with 3 (_Atomic(i32))
    if type_id(&ax) != 3 {
        panic!("Type IDs do not match");
    }

    // Compare the type ID of +cx with 1 (i32)
    if type_id(&(+cx)) != 1 {
        panic!("Type IDs do not match");
    }

    // Compare the type ID of +ax with 1 (i32)
    if type_id(&(+ax)) != 1 {
        panic!("Type IDs do not match");
    }

    // Assign x to y and compare
    let y = x;
    if y != 3 {
        panic!("Assignment failed");
    }

    // Increment x and compare
    x += 1;
    if x != 4 {
        panic!("Assignment failed");
    }

    // Create an instance of the S struct and compare
    let s: S = S { m: 7 };
    if s.m != 7 {
        panic!("Assignment failed");
    }

    // Assign a new value to s.m and compare
    s.m = 8;
    if s.m != 8 {
        panic!("Assignment failed");
    }
}

// Define a function to get the type ID of a value
fn type_id<T>(_: &T) -> i32
where
    T: std::any::Any,
{
    match T::any_cast::<i32>() {
        Some(1) => 1,
        Some(2) => 2,
        Some(3) => 3,
        _ => 99,
    }
}