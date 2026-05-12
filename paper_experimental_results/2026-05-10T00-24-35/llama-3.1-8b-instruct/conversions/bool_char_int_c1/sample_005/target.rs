// Define an enum E with two variants
enum E {
    E_NEG = -1,
    E_POS = 1,
}

// Define a macro TYPE_ID similar to the C version
// However, Rust does not support C's _Generic syntax directly.
macro_rules! TYPE_ID {
    ($val:expr) => {{
        match $val {
            _ => 99,
        }
    }};
}

// Define a function to assert the type of a value matches the expected type
fn assert_type<T, U>(got: T, want: U) -> Result<(), String>
where
    T: std::fmt::Debug,
    U: std::fmt::Debug,
{
    match got {
        T => match want {
            U => Ok(()),
            _ => Err(format!("Expected T, but got {:?}", want)),
        },
        _ => Err(format!("Expected {:?}, but got {:?}", T, got)),
    }
}

fn main() {
    // Test arithmetic with bool
    assert_type(TYPE_ID(1 + 0), 7).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });

    // Test arithmetic with char
    assert_type(TYPE_ID(1 + 0), 7).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(2);
    });

    // Test arithmetic with signed char
    assert_type(TYPE_ID(1 + 0), 7).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(3);
    });

    // Test arithmetic with unsigned char
    assert_type(TYPE_ID(1 + 0), 7).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(4);
    });

    // Test arithmetic with short
    assert_type(TYPE_ID(1 + 0), 7).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(5);
    });

    // Test arithmetic with unsigned short
    assert_type(TYPE_ID(1 + 0), 7).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(6);
    });

    // Test arithmetic with int and unsigned int
    assert_type(TYPE_ID(0 + 0u), 8).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(7);
    });

    // Test arithmetic with long and unsigned long
    assert_type(TYPE_ID(0 + 0u), 10).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(8);
    });

    // Test arithmetic with long long and unsigned long long
    assert_type(TYPE_ID(0 + 0u), 12).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(9);
    });

    // Test arithmetic with int and long
    assert_type(TYPE_ID(0 + 0), 9).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(10);
    });

    // Test arithmetic with long and long long
    assert_type(TYPE_ID(0 + 0), 11).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(11);
    });

    // Test arithmetic with int and long long
    assert_type(TYPE_ID(0 + 0), 11).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(12);
    });

    // Test arithmetic with enum E and int
    assert_type(TYPE_ID(0u + 0), TYPE_ID(0 + 0)).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(13);
    });

    // Test arithmetic with enum E and int
    assert_type(TYPE_ID(0 + 0), TYPE_ID(0 + 0)).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(14);
    });

    std::process::exit(0);
}