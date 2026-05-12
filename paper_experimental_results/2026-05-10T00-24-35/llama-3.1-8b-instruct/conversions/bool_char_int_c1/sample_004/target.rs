// Define an enum for E
enum E {
    E_NEG = -1,
    E_POS = 1,
}

// Define a function to determine the type of a value
fn type_id<T>(_: T) -> i32
where
    T: Copy + Eq + PartialEq + Ord + From<i8>,
{
    match std::mem::size_of::<T>() {
        1 => 2,
        2 => 5,
        4 => 7,
        8 => 9,
        _ => 99,
    }
}

// Define a function to check the type of two values
fn expect_type<T, U>(got: T, want: U) -> bool
where
    T: Eq + PartialEq + Copy + Ord,
    U: Eq + PartialEq + Copy + Ord,
{
    type_id(got) == type_id(want)
}

fn main() {
    // Test cases
    assert!(expect_type(1 + 0, 0), "1 + 0 should be int");
    assert!(expect_type(1 + 0, 0), "1 + 0 should be int");
    assert!(expect_type(b'1' + 0, 0), "b'1' + 0 should be int");
    assert!(expect_type(i8::MAX + 0, 0), "i8::MAX + 0 should be int");
    assert!(expect_type(u8::MAX + 0, 0), "u8::MAX + 0 should be int");
    assert!(expect_type(i16::MAX + 0, 0), "i16::MAX + 0 should be int");
    assert!(expect_type(u16::MAX + 0, 0), "u16::MAX + 0 should be int");
    assert!(expect_type(0 + 0u, 0), "0 + 0u should be int");
    assert!(expect_type(0 + 0u32, 0), "0 + 0u32 should be int");
    assert!(expect_type(0 + 0u64, 0), "0 + 0u64 should be int");
    assert!(expect_type(0 + 0i32, 0), "0 + 0i32 should be int");
    assert!(expect_type(0 + 0i64, 0), "0 + 0i64 should be int");
    assert!(expect_type((E::E_NEG as i8 + 0), (0 + 0)), "(E::E_NEG as i8 + 0) should be int");
    assert!(expect_type((E::E_NEG as i16 + 0), (0 + 0)), "(E::E_NEG as i16 + 0) should be int");
    assert!(expect_type((E::E_NEG as i32 + 0), (0 + 0)), "(E::E_NEG as i32 + 0) should be int");
    assert!(expect_type((E::E_NEG as i64 + 0), (0 + 0)), "(E::E_NEG as i64 + 0) should be int");

    println!("Success!");
}