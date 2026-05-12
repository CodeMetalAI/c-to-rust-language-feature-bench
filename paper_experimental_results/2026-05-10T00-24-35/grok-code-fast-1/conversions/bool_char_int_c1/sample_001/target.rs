use std::any::TypeId;
use std::ops::Add;

#[derive(Clone, Copy)]
struct Bool(bool);

#[derive(Clone, Copy)]
struct Char(i8);

#[derive(Clone, Copy)]
struct SignedChar(i8);

#[derive(Clone, Copy)]
struct UnsignedChar(u8);

#[derive(Clone, Copy)]
struct Short(i16);

#[derive(Clone, Copy)]
struct UnsignedShort(u16);

#[derive(Clone, Copy)]
struct Int(i32);

#[derive(Clone, Copy)]
struct UnsignedInt(u32);

#[derive(Clone, Copy)]
struct Long(i64);

#[derive(Clone, Copy)]
struct UnsignedLong(u64);

#[derive(Clone, Copy)]
struct LongLong(i64);

#[derive(Clone, Copy)]
struct UnsignedLongLong(u64);

#[derive(Clone, Copy)]
struct EnumE(i32);

fn type_id<T: 'static>() -> i32 {
    let tid = TypeId::of::<T>();
    if tid == TypeId::of::<Bool>() {
        1
    } else if tid == TypeId::of::<Char>() {
        2
    } else if tid == TypeId::of::<SignedChar>() {
        3
    } else if tid == TypeId::of::<UnsignedChar>() {
        4
    } else if tid == TypeId::of::<Short>() {
        5
    } else if tid == TypeId::of::<UnsignedShort>() {
        6
    } else if tid == TypeId::of::<Int>() {
        7
    } else if tid == TypeId::of::<UnsignedInt>() {
        8
    } else if tid == TypeId::of::<Long>() {
        9
    } else if tid == TypeId::of::<UnsignedLong>() {
        10
    } else if tid == TypeId::of::<LongLong>() {
        11
    } else if tid == TypeId::of::<UnsignedLongLong>() {
        12
    } else {
        99
    }
}

fn expect_type(got: i32, want: i32) -> bool {
    got == want
}

impl Add<i32> for Bool {
    type Output = Int;
    fn add(self, rhs: i32) -> Int {
        Int(self.0 as i32 + rhs)
    }
}

impl Add<i32> for Char {
    type Output = Int;
    fn add(self, rhs: i32) -> Int {
        Int(self.0 as i32 + rhs)
    }
}

impl Add<i32> for SignedChar {
    type Output = Int;
    fn add(self, rhs: i32) -> Int {
        Int(self.0 as i32 + rhs)
    }
}

impl Add<i32> for UnsignedChar {
    type Output = Int;
    fn add(self, rhs: i32) -> Int {
        Int(self.0 as i32 + rhs)
    }
}

impl Add<i32> for Short {
    type Output = Int;
    fn add(self, rhs: i32) -> Int {
        Int(self.0 as i32 + rhs)
    }
}

impl Add<i32> for UnsignedShort {
    type Output = Int;
    fn add(self, rhs: i32) -> Int {
        Int(self.0 as i32 + rhs)
    }
}

impl Add<UnsignedInt> for Int {
    type Output = UnsignedInt;
    fn add(self, rhs: UnsignedInt) -> UnsignedInt {
        UnsignedInt(self.0 as u32 + rhs.0)
    }
}

impl Add<UnsignedLong> for Long {
    type Output = UnsignedLong;
    fn add(self, rhs: UnsignedLong) -> UnsignedLong {
        UnsignedLong(self.0 as u64 + rhs.0)
    }
}

impl Add<UnsignedLongLong> for LongLong {
    type Output = UnsignedLongLong;
    fn add(self, rhs: UnsignedLongLong) -> UnsignedLongLong {
        UnsignedLongLong(self.0 as u64 + rhs.0)
    }
}

impl Add<Long> for Int {
    type Output = Long;
    fn add(self, rhs: Long) -> Long {
        Long(self.0 as i64 + rhs.0)
    }
}

impl Add<LongLong> for Long {
    type Output = LongLong;
    fn add(self, rhs: LongLong) -> LongLong {
        LongLong(self.0 + rhs.0)
    }
}

impl Add<LongLong> for Int {
    type Output = LongLong;
    fn add(self, rhs: LongLong) -> LongLong {
        LongLong(self.0 as i64 + rhs.0)
    }
}

impl Add<u32> for EnumE {
    type Output = UnsignedInt;
    fn add(self, rhs: u32) -> UnsignedInt {
        UnsignedInt(self.0 as u32 + rhs)
    }
}

impl Add<i32> for EnumE {
    type Output = Int;
    fn add(self, rhs: i32) -> Int {
        Int(self.0 + rhs)
    }
}

impl Add<u32> for Int {
    type Output = UnsignedInt;
    fn add(self, rhs: u32) -> UnsignedInt {
        UnsignedInt(self.0 as u32 + rhs)
    }
}

impl Add<i32> for Int {
    type Output = Int;
    fn add(self, rhs: i32) -> Int {
        Int(self.0 + rhs)
    }
}

fn main() -> i32 {
    if !expect_type(type_id::<<Bool as Add<i32>>::Output>(), 7) {
        return 1;
    }
    if !expect_type(type_id::<<Char as Add<i32>>::Output>(), 7) {
        return 2;
    }
    if !expect_type(type_id::<<SignedChar as Add<i32>>::Output>(), 7) {
        return 3;
    }
    if !expect_type(type_id::<<UnsignedChar as Add<i32>>::Output>(), 7) {
        return 4;
    }
    if !expect_type(type_id::<<Short as Add<i32>>::Output>(), 7) {
        return 5;
    }
    if !expect_type(type_id::<<UnsignedShort as Add<i32>>::Output>(), 7) {
        return 6;
    }
    if !expect_type(type_id::<<Int as Add<UnsignedInt>>::Output>(), 8) {
        return 7;
    }
    if !expect_type(type_id::<<Long as Add<UnsignedLong>>::Output>(), 10) {
        return 8;
    }
    if !expect_type(type_id::<<LongLong as Add<UnsignedLongLong>>::Output>(), 12) {
        return 9;
    }
    if !expect_type(type_id::<<Int as Add<Long>>::Output>(), 9) {
        return 10;
    }
    if !expect_type(type_id::<<Long as Add<LongLong>>::Output>(), 11) {
        return 11;
    }
    if !expect_type(type_id::<<Int as Add<LongLong>>::Output>(), 11) {
        return 12;
    }
    if !expect_type(
        type_id::<<EnumE as Add<u32>>::Output>(),
        type_id::<<Int as Add<u32>>::Output>(),
    ) {
        return 13;
    }
    if !expect_type(
        type_id::<<EnumE as Add<i32>>::Output>(),
        type_id::<<Int as Add<i32>>::Output>(),
    ) {
        return 14;
    }
    0
}