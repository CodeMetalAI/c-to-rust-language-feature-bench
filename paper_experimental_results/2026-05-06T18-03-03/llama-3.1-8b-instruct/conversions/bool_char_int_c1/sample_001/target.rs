// Define an enum to represent the type IDs
enum TypeId {
    Bool = 1,
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
    Default = 99,
}

// Define a function to get the type ID of a value
fn type_id<T>(x: T) -> TypeId
where
    T: Copy,
{
    match x {
        _ => TypeId::Default,
    }
}

// Define an enum to represent the test results
enum TestResult {
    Pass,
    Fail(u32),
}

// Define a function to check if the type ID of two values matches
fn expect_type<T, U>(got: TypeId, want: TypeId) -> TestResult
where
    T: Copy,
    U: Copy,
{
    if got == want {
        TestResult::Pass
    } else {
        TestResult::Fail(0)
    }
}

fn main() {
    // Test 1: bool + int
    let result = expect_type(type_id(1i8 as i32 + 0), TypeId::Int);
    match result {
        TestResult::Pass => (),
        TestResult::Fail(code) => {
            println!("Test 1 failed with code {}", code);
            std::process::exit(1);
        }
    }

    // Test 2: char + int
    let result = expect_type(type_id(1i8 as i32 + 0), TypeId::Int);
    match result {
        TestResult::Pass => (),
        TestResult::Fail(code) => {
            println!("Test 2 failed with code {}", code);
            std::process::exit(2);
        }
    }

    // Test 3: signed char + int
    let result = expect_type(type_id(1i8 as i32 + 0), TypeId::Int);
    match result {
        TestResult::Pass => (),
        TestResult::Fail(code) => {
            println!("Test 3 failed with code {}", code);
            std::process::exit(3);
        }
    }

    // Test 4: unsigned char + int
    let result = expect_type(type_id(1u8 as u32 + 0), TypeId::UnsignedInt);
    match result {
        TestResult::Pass => (),
        TestResult::Fail(code) => {
            println!("Test 4 failed with code {}", code);
            std::process::exit(4);
        }
    }

    // Test 5: short + int
    let result = expect_type(type_id(1i16 as i32 + 0), TypeId::Int);
    match result {
        TestResult::Pass => (),
        TestResult::Fail(code) => {
            println!("Test 5 failed with code {}", code);
            std::process::exit(5);
        }
    }

    // Test 6: unsigned short + int
    let result = expect_type(type_id(1u16 as u32 + 0), TypeId::UnsignedInt);
    match result {
        TestResult::Pass => (),
        TestResult::Fail(code) => {
            println!("Test 6 failed with code {}", code);
            std::process::exit(6);
        }
    }

    // Test 7: int + unsigned int
    let result = expect_type(type_id(0i32 + 0u32), TypeId::UnsignedInt);
    match result {
        TestResult::Pass => (),
        TestResult::Fail(code) => {
            println!("Test 7 failed with code {}", code);
            std::process::exit(7);
        }
    }

    // Test 8: long + unsigned long
    let result = expect_type(type_id(0i64 + 0u64), TypeId::UnsignedLong);
    match result {
        TestResult::Pass => (),
        TestResult::Fail(code) => {
            println!("Test 8 failed with code {}", code);
            std::process::exit(8);
        }
    }

    // Test 9: long long + unsigned long long
    let result = expect_type(type_id(0i64 + 0u64), TypeId::UnsignedLongLong);
    match result {
        TestResult::Pass => (),
        TestResult::Fail(code) => {
            println!("Test 9 failed with code {}", code);
            std::process::exit(9);
        }
    }

    // Test 10: int + long
    let result = expect_type(type_id(0i32 + 0i64), TypeId::Long);
    match result {
        TestResult::Pass => (),
        TestResult::Fail(code) => {
            println!("Test 10 failed with code {}", code);
            std::process::exit(10);
        }
    }

    // Test 11: long + long long
    let result = expect_type(type_id(0i64 + 0i64), TypeId::LongLong);
    match result {
        TestResult::Pass => (),
        TestResult::Fail(code) => {
            println!("Test 11 failed with code {}", code);
            std::process::exit(11);
        }
    }

    // Test 12: int + long long
    let result = expect_type(type_id(0i32 + 0i64), TypeId::LongLong);
    match result {
        TestResult::Pass => (),
        TestResult::Fail(code) => {
            println!("Test 12 failed with code {}", code);
            std::process::exit(12);
        }
    }

    // Test 13: enum + int
    let result = expect_type(type_id(0i32 as i32 + 0), type_id(0i32 + 0));
    match result {
        TestResult::Pass => (),
        TestResult::Fail(code) => {
            println!("Test 13 failed with code {}", code);
            std::process::exit(13);
        }
    }

    // Test 14: enum + int
    let result = expect_type(type_id(0i32 as i32 + 0), type_id(0i32 + 0));
    match result {
        TestResult::Pass => (),
        TestResult::Fail(code) => {
            println!("Test 14 failed with code {}", code);
            std::process::exit(14);
        }
    }

    println!("All tests passed");
    std::process::exit(0);
}