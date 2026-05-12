#[cfg(test)]
mod tests {
    #[test]
    fn test_alignment() {
        #[repr(packed)]
        struct BoolStruct {
            c: u8,
            v: bool,
        }
        assert!(std::mem::align_of::<bool>() <= std::mem::align_of::<BoolStruct>());

        #[repr(packed)]
        struct CharStruct {
            c: u8,
            v: char,
        }
        assert!(std::mem::align_of::<char>() <= std::mem::align_of::<CharStruct>());

        #[repr(packed)]
        struct SignedCharStruct {
            c: u8,
            v: i8,
        }
        assert!(std::mem::align_of::<i8>() <= std::mem::align_of::<SignedCharStruct>());

        #[repr(packed)]
        struct UnsignedCharStruct {
            c: u8,
            v: u8,
        }
        assert!(std::mem::align_of::<u8>() <= std::mem::align_of::<UnsignedCharStruct>());

        #[repr(packed)]
        struct SignedShortStruct {
            c: u8,
            v: i16,
        }
        assert!(std::mem::align_of::<i16>() <= std::mem::align_of::<SignedShortStruct>());

        #[repr(packed)]
        struct UnsignedShortStruct {
            c: u8,
            v: u16,
        }
        assert!(std::mem::align_of::<u16>() <= std::mem::align_of::<UnsignedShortStruct>());

        #[repr(packed)]
        struct SignedIntStruct {
            c: u8,
            v: i32,
        }
        assert!(std::mem::align_of::<i32>() <= std::mem::align_of::<SignedIntStruct>());

        #[repr(packed)]
        struct UnsignedIntStruct {
            c: u8,
            v: u32,
        }
        assert!(std::mem::align_of::<u32>() <= std::mem::align_of::<UnsignedIntStruct>());

        #[repr(packed)]
        struct SignedLongStruct {
            c: u8,
            v: i64,
        }
        assert!(std::mem::align_of::<i64>() <= std::mem::align_of::<SignedLongStruct>());

        #[repr(packed)]
        struct UnsignedLongStruct {
            c: u8,
            v: u64,
        }
        assert!(std::mem::align_of::<u64>() <= std::mem::align_of::<UnsignedLongStruct>());

        #[repr(packed)]
        struct SignedLongLongStruct {
            c: u8,
            v: i128,
        }
        assert!(std::mem::align_of::<i128>() <= std::mem::align_of::<SignedLongLongStruct>());

        #[repr(packed)]
        struct UnsignedLongLongStruct {
            c: u8,
            v: u128,
        }
        assert!(std::mem::align_of::<u128>() <= std::mem::align_of::<UnsignedLongLongStruct>());

        #[repr(packed)]
        struct FloatStruct {
            c: u8,
            v: f32,
        }
        assert!(std::mem::align_of::<f32>() <= std::mem::align_of::<FloatStruct>());

        #[repr(packed)]
        struct DoubleStruct {
            c: u8,
            v: f64,
        }
        assert!(std::mem::align_of::<f64>() <= std::mem::align_of::<DoubleStruct>());

        #[repr(packed)]
        struct LongDoubleStruct {
            c: u8,
            v: f128,
        }
        assert!(std::mem::align_of::<f128>() <= std::mem::align_of::<LongDoubleStruct>());

        #[repr(packed)]
        struct ComplexFloatStruct {
            c: u8,
            v: [f32; 2],
        }
        assert!(std::mem::align_of::<[f32; 2]>() <= std::mem::align_of::<ComplexFloatStruct>());

        #[repr(packed)]
        struct ComplexDoubleStruct {
            c: u8,
            v: [f64; 2],
        }
        assert!(std::mem::align_of::<[f64; 2]>() <= std::mem::align_of::<ComplexDoubleStruct>());

        #[repr(packed)]
        struct ComplexLongDoubleStruct {
            c: u8,
            v: [f128; 2],
        }
        assert!(std::mem::align_of::<[f128; 2]>() <= std::mem::align_of::<ComplexLongDoubleStruct>());
    }
}

fn main() {
    tests::test_alignment();
}