#[allow(unused_macros)]
macro_rules! type_id {
    ($expr:expr) => {{
        match $expr {
            _ if is_array($expr) => 2,
            _ if is_array_mut($expr) => 2,
            _ if is_string_slice($expr) => 3,
            _ if is_string_slice_const($expr) => 4,
            _ => 0,
        }
    }};
}

fn is_array<T>(_expr: &T) -> bool
where
    T: std::array::FromIter<[u8; 0]>,
{
    std::mem::size_of_val(&$expr) == std::mem::size_of::<T>()
}

fn is_array_mut<T>(_expr: &mut T) -> bool
where
    T: std::array::FromIter<[u8; 0]>,
{
    std::mem::size_of_val(&$expr) == std::mem::size_of::<T>()
}

fn is_string_slice<T>(_expr: &T) -> bool
where
    T: std::string::String + std::ops::Deref<Target = str>,
{
    std::mem::size_of_val(&$expr) == std::mem::size_of::<String>()
}

fn is_string_slice_const<T>(_expr: &T) -> bool
where
    T: std::string::String + std::ops::Deref<Target = str>,
{
    std::mem::size_of_val(&$expr) == std::mem::size_of::<&str>()
}

fn main() {
    let a = [10, 20, 30];
    if type_id(&a) != 2 {
        return;
    }
    if type_id(&a) != 2 {
        return 2;
    }

    if a[0] != 10 {
        return 3;
    }
    if a[1] != 20 {
        return 4;
    }
    if a[2] != 30 {
        return 5;
    }

    if a.len() != 3 {
        return 6;
    }
    if std::mem::align_of::<u8>() != std::mem::align_of::<usize>() {
        return 7;
    }

    let s = "abc";
    if type_id(&s) != 4 {
        return 8;
    }
    if s.chars().next().unwrap() != 'a' {
        return 9;
    }
    if s.chars().nth(1).unwrap() != 'b' {
        return 10;
    }
    if s.chars().nth(2).unwrap() != 'c' {
        return 11;
    }
    if s.chars().nth(3).unwrap() != '\0' {
        return 12;
    }

    return 0;
}