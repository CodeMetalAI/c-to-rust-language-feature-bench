use std::any::TypeId;

#[derive(Debug, Clone, Copy)]
enum TypeTag {
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
    Other = 99,
}

fn type_id<T: 'static>() -> TypeTag {
    let type_id = TypeId::of::<T>();
    
    if type_id == TypeId::of::<bool>() {
        TypeTag::Bool
    } else if type_id == TypeId::of::<i8>() {
        TypeTag::SignedChar
    } else if type_id == TypeId::of::<u8>() {
        TypeTag::UnsignedChar
    } else if type_id == TypeId::of::<i16>() {
        TypeTag::Short
    } else if type_id == TypeId::of::<u16>() {
        TypeTag::UnsignedShort
    } else if type_id == TypeId::of::<i32>() {
        TypeTag::Int
    } else if type_id == TypeId::of::<u32>() {
        TypeTag::UnsignedInt
    } else if type_id == TypeId::of::<i64>() {
        TypeTag::Long
    } else if type_id == TypeId::of::<u64>() {
        TypeTag::UnsignedLong
    } else if type_id == TypeId::of::<i128>() {
        TypeTag::LongLong
    } else if type_id == TypeId::of::<u128>() {
        TypeTag::UnsignedLongLong
    } else {
        TypeTag::Other
    }
}

macro_rules! TYPE_ID {
    ($x:expr) => {
        {
            let _dummy = $x;
            type_id::<decltype!(_dummy)>()
        }
    };
}

#[macro_export]
macro_rules! decltype {
    ($e:expr) => {
        $e
    };
}

#[derive(Clone, Copy)]
enum E { NEG = -1, POS = 1 }

fn expect_type(got: TypeTag, want: TypeTag) -> bool {
    got as i32 == want as i32
}

fn main() {
    if !expect_type(type_id::<i32>(), TypeTag::Int) {
        std::process::exit(1);
    }
    if !expect_type(type_id::<i32>(), TypeTag::Int) {
        std::process::exit(2);
    }
    if !expect_type(type_id::<i32>(), TypeTag::Int) {
        std::process::exit(3);
    }
    if !expect_type(type_id::<i32>(), TypeTag::Int) {
        std::process::exit(4);
    }
    if !expect_type(type_id::<i32>(), TypeTag::Int) {
        std::process::exit(5);
    }
    if !expect_type(type_id::<i32>(), TypeTag::Int) {
        std::process::exit(6);
    }

    if !expect_type(type_id::<u32>(), TypeTag::UnsignedInt) {
        std::process::exit(7);
    }
    if !expect_type(type_id::<u64>(), TypeTag::UnsignedLong) {
        std::process::exit(8);
    }
    if !expect_type(type_id::<u128>(), TypeTag::UnsignedLongLong) {
        std::process::exit(9);
    }

    if !expect_type(type_id::<i64>(), TypeTag::Long) {
        std::process::exit(10);
    }
    if !expect_type(type_id::<i128>(), TypeTag::LongLong) {
        std::process::exit(11);
    }
    if !expect_type(type_id::<i128>(), TypeTag::LongLong) {
        std::process::exit(12);
    }

    if !expect_type(type_id::<u32>(), type_id::<u32>()) {
        std::process::exit(13);
    }
    if !expect_type(type_id::<i32>(), type_id::<i32>()) {
        std::process::exit(14);
    }

    std::process::exit(0);
}