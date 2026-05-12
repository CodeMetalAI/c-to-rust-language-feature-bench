// Define an enum to map type IDs
enum TypeId {
    Int = 1,
    UInt = 2,
    Long = 3,
    ULong = 4,
    LongLong = 5,
    ULongLong = 6,
    Float = 7,
    Double = 8,
    LongDouble = 9,
}

// Define a struct BF
struct Bf {
    u1: u8, // unsigned int
    i1: i8, // int
    b1: bool, // _Bool
}

fn main() {
    // Test signed char
    let type_id = TypeId::TypeIdFromS32(signed_char(1) + 0);
    if type_id != TypeId::Int {
        return 1;
    }

    // Test unsigned char
    let type_id = TypeId::TypeIdFromU32(unsigned_char(1) + 0);
    if type_id != TypeId::UInt {
        return 2;
    }

    // Test short
    let type_id = TypeId::TypeIdFromS32(short(1) + 0);
    if type_id != TypeId::Int {
        return 3;
    }

    // Test unsigned short
    let type_id = TypeId::TypeIdFromU32(unsigned_short(1) + 0);
    if type_id != TypeId::UInt {
        return 4;
    }

    // Create a Bf instance and test its fields
    let bf = Bf {
        u1: 1u8,
        i1: -1i8,
        b1: true,
    };

    let type_id = TypeId::TypeIdFromU8(bf.u1 + 0);
    if type_id != TypeId::UInt {
        return 5;
    }
    let type_id = TypeId::TypeIdFromI8(bf.i1 + 0);
    if type_id != TypeId::Int {
        return 6;
    }
    let type_id = TypeId::TypeIdFromBool(bf.b1 + 0);
    if type_id != TypeId::Int {
        return 7;
    }

    // Test float, double, and long double
    let type_id = TypeId::TypeIdFromF32(float(1.0f32));
    if type_id != TypeId::Float {
        return 8;
    }
    let type_id = TypeId::TypeIdFromF64(double(1.0));
    if type_id != TypeId::Double {
        return 9;
    }
    let type_id = TypeId::TypeIdFromF64(long_double(1.0));
    if type_id != TypeId::LongDouble {
        return 10;
    }

    // If all tests pass, return 0
    return 0;
}

// Helper traits to convert primitive types to enum TypeId
trait ToTypeId {
    fn to_type_id(&self) -> TypeId;
}

impl ToTypeId for u8 {
    fn to_type_id(&self) -> TypeId {
        TypeId::UInt
    }
}

impl ToTypeId for char {
    fn to_type_id(&self) -> TypeId {
        TypeId::UInt
    }
}

impl ToTypeId for i8 {
    fn to_type_id(&self) -> TypeId {
        TypeId::Int
    }
}

impl ToTypeId for bool {
    fn to_type_id(&self) -> TypeId {
        TypeId::Int
    }
}

impl ToTypeId for i16 {
    fn to_type_id(&self) -> TypeId {
        TypeId::Int
    }
}

impl ToTypeId for u16 {
    fn to_type_id(&self) -> TypeId {
        TypeId::UInt
    }
}

impl ToTypeId for i32 {
    fn to_type_id(&self) -> TypeId {
        TypeId::Int
    }
}

impl ToTypeId for u32 {
    fn to_type_id(&self) -> TypeId {
        TypeId::UInt
    }
}

impl ToTypeId for f32 {
    fn to_type_id(&self) -> TypeId {
        TypeId::Float
    }
}

impl ToTypeId for f64 {
    fn to_type_id(&self) -> TypeId {
        TypeId::Double
    }
}

// Helper functions to get the type ID from primitive types
fn signed_char(x: i8) -> i8 {
    x
}

fn unsigned_char(x: u8) -> u8 {
    x
}

fn short(x: i16) -> i16 {
    x
}

fn unsigned_short(x: u16) -> u16 {
    x
}

fn float(x: f32) -> f32 {
    x
}

fn double(x: f64) -> f64 {
    x
}

fn long_double(x: f64) -> f64 {
    x
}

// Implement TypeId::TypeIdFrom for primitive types
impl TypeId {
    fn TypeIdFromS8(x: i8) -> TypeId {
        x.to_type_id()
    }

    fn TypeIdFromU8(x: u8) -> TypeId {
        x.to_type_id()
    }

    fn TypeIdFromS16(x: i16) -> TypeId {
        x.to_type_id()
    }

    fn TypeIdFromU16(x: u16) -> TypeId {
        x.to_type_id()
    }

    fn TypeIdFromS32(x: i32) -> TypeId {
        x.to_type_id()
    }

    fn TypeIdFromU32(x: u32) -> TypeId {
        x.to_type_id()
    }

    fn TypeIdFromF32(x: f32) -> TypeId {
        x.to_type_id()
    }

    fn TypeIdFromF64(x: f64) -> TypeId {
        x.to_type_id()
    }

    fn TypeIdFromBool(x: bool) -> TypeId {
        x.to_type_id()
    }
}