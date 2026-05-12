// type_id.rs

#[derive(Debug, PartialEq)]
enum TypeId {
    Int,
    Uint,
    Long,
    Ulong,
    LongLong,
    UlongLong,
    Float,
    Double,
    LongDouble,
    Other,
}

macro_rules! type_id {
    ($x:expr) => {{
        let x = $x;
        match x {
            _ if x.is_int() => TypeId::Int,
            _ if x.is_uint() => TypeId::Uint,
            _ if x.is_long() => TypeId::Long,
            _ if x.is_ulong() => TypeId::Ulong,
            _ if x.is_long_long() => TypeId::LongLong,
            _ if x.is_ulong_long() => TypeId::UlongLong,
            _ if x.is_float() => TypeId::Float,
            _ if x.is_double() => TypeId::Double,
            _ if x.is_long_double() => TypeId::LongDouble,
            _ => TypeId::Other,
        }
    }};
}

struct BF {
    u1: u8,
    i1: i8,
    b1: bool,
}

impl BF {
    fn new() -> Self {
        BF {
            u1: 1,
            i1: -1,
            b1: true,
        }
    }
}

fn main() {
    if type_id!(1i8 as i8 + 0)!= TypeId::Int {
        eprintln!("Error: Expected Int, got {:?}", type_id!(1i8 as i8 + 0));
        std::process::exit(1);
    }
    if type_id!(1u8 as u8 + 0)!= TypeId::Uint {
        eprintln!("Error: Expected Uint, got {:?}", type_id!(1u8 as u8 + 0));
        std::process::exit(2);
    }
    if type_id!(1i16 as i16 + 0)!= TypeId::Int {
        eprintln!("Error: Expected Int, got {:?}", type_id!(1i16 as i16 + 0));
        std::process::exit(3);
    }
    if type_id!(1u16 as u16 + 0)!= TypeId::Uint {
        eprintln!("Error: Expected Uint, got {:?}", type_id!(1u16 as u16 + 0));
        std::process::exit(4);
    }

    let mut bf = BF::new();
    if type_id!(bf.u1 + 0)!= TypeId::Uint {
        eprintln!("Error: Expected Uint, got {:?}", type_id!(bf.u1 + 0));
        std::process::exit(5);
    }
    if type_id!(bf.i1 + 0)!= TypeId::Int {
        eprintln!("Error: Expected Int, got {:?}", type_id!(bf.i1 + 0));
        std::process::exit(6);
    }
    if type_id!(bf.b1 + 0)!= TypeId::Int {
        eprintln!("Error: Expected Int, got {:?}", type_id!(bf.b1 + 0));
        std::process::exit(7);
    }

    if type_id!(1.0f32)!= TypeId::Float {
        eprintln!("Error: Expected Float, got {:?}", type_id!(1.0f32));
        std::process::exit(8);
    }
    if type_id!(1.0f64)!= TypeId::Double {
        eprintln!("Error: Expected Double, got {:?}", type_id!(1.0f64));
        std::process::exit(9);
    }
    if type_id!(1.0f80)!= TypeId::LongDouble {
        eprintln!("Error: Expected LongDouble, got {:?}", type_id!(1.0f80));
        std::process::exit(10);
    }

    println!("Success!");
}

trait IntLike {
    fn is_int(&self) -> bool;
}

impl IntLike for i8 {
    fn is_int(&self) -> bool {
        true
    }
}

impl IntLike for i16 {
    fn is_int(&self) -> bool {
        true
    }
}

impl IntLike for i32 {
    fn is_int(&self) -> bool {
        true
    }
}

impl IntLike for i64 {
    fn is_int(&self) -> bool {
        true
    }
}

trait UintLike {
    fn is_uint(&self) -> bool;
}

impl UintLike for u8 {
    fn is_uint(&self) -> bool {
        true
    }
}

impl UintLike for u16 {
    fn is_uint(&self) -> bool {
        true
    }
}

impl UintLike for u32 {
    fn is_uint(&self) -> bool {
        true
    }
}

impl UintLike for u64 {
    fn is_uint(&self) -> bool {
        true
    }
}

trait LongLike {
    fn is_long(&self) -> bool;
}

impl LongLike for i64 {
    fn is_long(&self) -> bool {
        true
    }
}

impl LongLike for u64 {
    fn is_long(&self) -> bool {
        true
    }
}

trait UlongLike {
    fn is_ulong(&self) -> bool;
}

impl UlongLike for i64 {
    fn is_ulong(&self) -> bool {
        true
    }
}

impl UlongLike for u64 {
    fn is_ulong(&self) -> bool {
        true
    }
}

trait LongLongLike {
    fn is_long_long(&self) -> bool;
}

impl LongLongLike for i64 {
    fn is_long_long(&self) -> bool {
        true
    }
}

impl LongLongLike for u64 {
    fn is_long_long(&self) -> bool {
        true
    }
}

trait UlongLongLike {
    fn is_ulong_long(&self) -> bool;
}

impl UlongLongLike for i64 {
    fn is_ulong_long(&self) -> bool {
        true
    }
}

impl UlongLongLike for u64 {
    fn is_ulong_long(&self) -> bool {
        true
    }
}

trait FloatLike {
    fn is_float(&self) -> bool;
}

impl FloatLike for f32 {
    fn is_float(&self) -> bool {
        true
    }
}

impl FloatLike for f64 {
    fn is_float(&self) -> bool {
        true
    }
}

impl FloatLike for f80 {
    fn is_float(&self) -> bool {
        true
    }
}

trait DoubleLike {
    fn is_double(&self) -> bool;
}

impl DoubleLike for f64 {
    fn is_double(&self) -> bool {
        true
    }
}

trait LongDoubleLike {
    fn is_long_double(&self) -> bool;
}

impl LongDoubleLike for f80 {
    fn is_long_double(&self) -> bool {
        true
    }
}