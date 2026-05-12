#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum E {
    E_NEG = -1,
    E_POS = 1,
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

fn main() {
    if !expect_type(type_id((_Bool)1 + 0), 7) {
        std::process::exit(1);
    }
    if !expect_type(type_id((char)1 + 0), 7) {
        std::process::exit(2);
    }
    if !expect_type(type_id((signed char)1 + 0), 7) {
        std::process::exit(3);
    }
    if !expect_type(type_id((unsigned char)1 + 0), 7) {
        std::process::exit(4);
    }
    if !expect_type(type_id((short)1 + 0), 7) {
        std::process::exit(5);
    }
    if !expect_type(type_id((unsigned short)1 + 0), 7) {
        std::process::exit(6);
    }

    if !expect_type(type_id((int)0 + (unsigned int)0), 8) {
        std::process::exit(7);
    }
    if !expect_type(type_id((long)0 + (unsigned long)0), 10) {
        std::process::exit(8);
    }
    if !expect_type(type_id((long long)0 + (unsigned long long)0), 12) {
        std::process::exit(9);
    }

    if !expect_type(type_id((int)0 + (long)0), 9) {
        std::process::exit(10);
    }
    if !expect_type(type_id((long)0 + (long long)0), 11) {
        std::process::exit(11);
    }
    if !expect_type(type_id((int)0 + (long long)0), 11) {
        std::process::exit(12);
    }

    if !expect_type(type_id((enum E)0 + 0u), type_id((int)0 + 0u)) {
        std::process::exit(13);
    }
    if !expect_type(type_id((enum E)0 + 0), type_id((int)0 + 0)) {
        std::process::exit(14);
    }

    std::process::exit(0);
}

fn type_id<T>(x: T) -> i32 {
    let t = std::any::type_name::<T>();
    match t {
        "_Bool" => 1,
        "char" => 2,
        "signed char" => 3,
        "unsigned char" => 4,
        "short" => 5,
        "unsigned short" => 6,
        "int" => 7,
        "unsigned int" => 8,
        "long" => 9,
        "unsigned long" => 10,
        "long long" => 11,
        "unsigned long long" => 12,
        _ => 99,
    }
}