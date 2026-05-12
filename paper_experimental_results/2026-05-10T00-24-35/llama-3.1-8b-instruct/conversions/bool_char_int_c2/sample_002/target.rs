enum TypeId {
    Int = 1,
    UnsignedInt = 2,
    Long = 3,
    UnsignedLong = 4,
    LongLong = 5,
    UnsignedLongLong = 6,
    Float = 7,
    Double = 8,
    LongDouble = 9,
    Default = 99,
}

#[repr(packed)]
struct BF {
    u1: u8,
    i1: i8,
    b1: bool,
}

fn main() {
    if TYPE_ID((char as i32 + 0) as u32 != 1) {
        return 1;
    }
    if TYPE_ID((u8 + 0) as u32 != 1) {
        return 2;
    }
    if TYPE_ID((i16 + 0) as u32 != 1) {
        return 3;
    }
    if TYPE_ID((u16 + 0) as u32 != 1) {
        return 4;
    }

    let mut bf = BF { u1: 1, i1: -1 as i8, b1: true };
    if TYPE_ID(bf.u1 as u32 + 0) as u32 != 1 {
        return 5;
    }
    if TYPE_ID(bf.i1 as u32 + 0) as u32 != 1 {
        return 6;
    }
    if TYPE_ID(bf.b1 as u32 + 0) as u32 != 1 {
        return 7;
    }

    if TYPE_ID(1.0 as f32 as u32) != 7 {
        return 8;
    }
    if TYPE_ID(1.0 as f64 as u32) != 8 {
        return 9;
    }
    if TYPE_ID(1.0 as f64 as u64 as u32) != 9 {
        return 10;
    }

    0
}

fn TYPE_ID<T>(x: T) -> u32 {
    match x {
        _ if std::is_same::<T, i32>() => TypeId::Int,
        _ if std::is_same::<T, u32>() => TypeId::UnsignedInt,
        _ if std::is_same::<T, i64>() => TypeId::Long,
        _ if std::is_same::<T, u64>() => TypeId::UnsignedLong,
        _ if std::is_same::<T, i64>() => TypeId::LongLong,
        _ if std::is_same::<T, u64>() => TypeId::UnsignedLongLong,
        _ if std::is_same::<T, f32>() => TypeId::Float,
        _ if std::is_same::<T, f64>() => TypeId::Double,
        _ if std::is_same::<T, f64>() => TypeId::LongDouble,
        _ => TypeId::Default,
    } as u32
}