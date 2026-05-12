// Define an enum to map various integer types to a number
enum IntType {
    Boolean = 1,
    Char = 2,
    SignedChar = 3,
    UnsignedChar = 4,
    Short = 5,
    UnsignedShort = 6,
    Int = 7,
    UnsignedInt = 8,
    Long = 9,
    UnsignedLong = 10,
    LongLong = 11,
    UnsignedLongLong = 12,
}

fn type_id<T>(_: T) -> i32
where
    T: Copy + Eq + std::ops::Add<Output = T>,
{
    match T::default() {
        _ => IntType::Int as i32,
    }
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn main() {
    // Test different types in arithmetic expressions
    if !expect_type(type_id(_ => 1u8 + 0u8 as u8), 7) {
        println!("Test 1 failed");
        return 1;
    }
    if !expect_type(type_id(_ => 0i16 + 0u16), 7) {
        println!("Test 2 failed");
        return 2;
    }
    if !expect_type(type_id(_ => 0isize + 0usize), 7) {
        println!("Test 3 failed");
        return 3;
    }

    // Test mixed-type arithmetic expressions
    if !expect_type(type_id(_ => 0i32 + 0u32), 8) {
        println!("Test 4 failed");
        return 4;
    }
    if !expect_type(type_id(_ => 0i64 + 0u64), 12) {
        println!("Test 5 failed");
        return 5;
    }

    // Test mixed-type arithmetic expressions with long and long long types
    if !expect_type(type_id(_ => 0i32 + 0i64), 11) {
        println!("Test 6 failed");
        return 6;
    }
    if !expect_type(type_id(_ => 0i64 + 0i64), 11) {
        println!("Test 7 failed");
        return 7;
    }
    if !expect_type(type_id(_ => 0i32 + 0i64), 11) {
        println!("Test 8 failed");
        return 8;
    }

    // Test enum with integer value in arithmetic expressions
    if !expect_type(type_id(_ => enum Foo { A = 0 }::A + 0u8 as enum Foo), 7) {
        println!("Test 9 failed");
        return 9;
    }
    if !expect_type(type_id(_ => enum Foo { A = 0 }::A + 0u16 as enum Foo), 7) {
        println!("Test 10 failed");
        return 10;
    }

    println!("All tests passed");
    // Return 0 instead of returning from main
    std::process::exit(0);
}

// Define an enum for testing
enum Foo {
    A = 0,
}