// Define a macro to determine the type ID
macro_rules! type_id {
    ($x:expr) => {{
        match $x {
            _ if is_signed($x) => 1,
            _ if is_unsigned($x) => 2,
            _ if is_float($x) => 7,
            _ if is_double($x) => 8,
            _ if is_long_double($x) => 9,
            _ => 99,
        }
    }};
}

// Define a struct to represent the BF type
#[repr(C)]
struct BF {
    u1: u32,
    i1: i32,
    b1: bool,
}

fn main() {
    // Test the type ID macro with various types
    if type_id((1 as i8) + 0) != 1 {
        return 1;
    }
    if type_id((1 as u8) + 0) != 1 {
        return 2;
    }
    if type_id((1 as i16) + 0) != 1 {
        return 3;
    }
    if type_id((1 as u16) + 0) != 1 {
        return 4;
    }

    // Create a BF struct and test its type ID
    let mut bf = BF {
        u1: 1,
        i1: -1,
        b1: true,
    };

    if type_id(bf.u1 + 0) != 1 {
        return 5;
    }
    if type_id(bf.i1 + 0) != 1 {
        return 6;
    }
    if type_id(bf.b1 as i32 + 0) != 1 {
        return 7;
    }

    // Test the type ID macro with floating-point types
    if type_id(1.0f32) != 7 {
        return 8;
    }
    if type_id(1.0f64) != 8 {
        return 9;
    }
    if type_id(1.0f80) != 9 {
        return 10;
    }

    // If all tests pass, return 0
    return 0;
}

// Helper functions to determine the type of a value
fn is_signed<T: std::ops::BitOrAssign + Copy>(x: T) -> bool {
    x & 0x8000_0000 != 0
}

fn is_unsigned<T: std::ops::BitOrAssign + Copy>(x: T) -> bool {
    !is_signed(x)
}

fn is_float<T: std::ops::BitOrAssign + Copy>(x: T) -> bool {
    x & 0x7f800000 != 0
}

fn is_double<T: std::ops::BitOrAssign + Copy>(x: T) -> bool {
    x & 0xff800000 != 0
}

fn is_long_double<T: std::ops::BitOrAssign + Copy>(x: T) -> bool {
    x & 0xffc00000 != 0
}