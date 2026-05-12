// Test Rust alignment support.

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::panic;

    #[test]
    fn test_alignas() {
        assert_eq!(alignas::Alignas::alignof::<alignas::Alignas>(), 1);
        assert_eq!(alignas::Alignas::alignof::<std::max_align_t>(), 1);

        let s = alignas::Alignas::new();
        assert_eq!(s.alignof::<alignas::Alignas>(), 1);
        assert_eq!(s.alignof::<std::max_align_t>(), 1);

        let c = [1u8; 1];
        assert_eq!(alignas::Alignas::alignof::<[u8; 1]>(), 1);

        let _x = alignas::Alignas::new();
        assert_eq!(alignas::Alignas::alignof::<_x>(), 1);

        panic::assert_panic!(|| {
            let invalid: alignas::Alignas = alignas::Alignas::new();
            invalid.alignof::<u8>()
        });
    }
}

#[derive(Debug, PartialEq)]
enum Alignas {
    Alignas,
    Alignof,
}

impl std::fmt::Display for Alignas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Alignas::Alignas => write!(f, "_Alignas"),
            Alignas::Alignof => write!(f, "_Alignof"),
        }
    }
}

impl std::str::FromStr for Alignas {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "_Alignas" => Ok(Alignas::Alignas),
            "_Alignof" => Ok(Alignas::Alignof),
            _ => Err(format!("invalid alignas: {}", s)),
        }
    }
}

impl std::default::Default for Alignas {
    fn default() -> Self {
        Alignas::Alignas
    }
}

fn main() {
    let s1 = std::mem::size_of::<Alignas>().to_string();
    let s2 = std::mem::align_of::<std::max_align_t>().to_string();
    let s3 = "1".to_string();
    let s4 = "1".to_string();

    assert_eq!(s1, "_Alignas");
    assert_eq!(s2, "_Alignof");
    assert_eq!(s3, "1");
    assert_eq!(s4, "1");
}