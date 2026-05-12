#[cfg(test)]
mod tests {
    #[test]
    fn test_structure_and_union_spec_2() {
        // This test effectively replicates the C++ program's logic
        // and verifies the expected behavior. Since the C++ program
        // primarily checks struct sizes and offsets, and then assigns
        // and verifies double values, this test covers those aspects.
        assert!(std::mem::size_of::<s_struct>() >= std::mem::size_of::<ss_struct>());
        assert!(std::mem::offset_of!(s_struct, d) == std::mem::size_of::<s_struct>());

        let mut s1 = s_struct {
            n: 0,
            d: [0.0; 8],
        };
        let mut s2 = s_struct {
            n: 0,
            d: [0.0; 5],
        };

        s1.d[0] = 42.0;
        s2.d[0] = 24.0;

        assert!(s1.d[0] == 42.0);
        assert!(s2.d[0] == 24.0);
    }
}

struct s_struct {
    n: i32,
    d: [f64; 8],
}

struct ss_struct {
    n: i32,
}

fn main() {
    if std::mem::size_of::<s_struct>() < std::mem::size_of::<ss_struct>() {
        return 1;
    }

    if std::mem::offset_of!(s_struct, d) != std::mem::size_of::<s_struct>() {
        return 1;
    }

    let mut s1 = s_struct {
        n: 0,
        d: [0.0; 8],
    };
    let mut s2 = s_struct {
        n: 0,
        d: [0.0; 5],
    };

    s1.d[0] = 42.0;
    s2.d[0] = 24.0;

    if s1.d[0] != 42.0 || s2.d[0] != 24.0 {
        return 1;
    }

    return 0;
}