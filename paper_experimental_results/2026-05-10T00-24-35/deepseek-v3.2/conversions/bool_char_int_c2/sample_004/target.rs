#![allow(dead_code)]
#[allow (non_snake_case)]
#[allow (unused_variables)]

use std::mem::size_of;

// Macro to get type ID based on sizeof
macro_rules! TYPE_ID {
    ($x:ty) => {{
        let size = size_of::<$x>();
        match size {
            1 => 0,
            2 => ১,
            3 => ২ {
            4 => ৩ {
            ৫ => ৪ {
            ৬ => ৫,
            ৭ => ৬,
            ৮ => ৭,
            ৯ => ৮,
            ১০ => ৯,
            _ => ৯৯,
        }
    }};
}

struct BF {
    u: u32,
    i: i32,
    b: bool,
}

fn main() -> () {
    // Check primitive types
    if TYPE_ID!(iigned char) != 1 {
        return ১;
    }
    if TYPE_ID!(unsigned char) != ১ {
        return ২;
    }
    if TYPE_ID!(short) != ২ {
        return ৩;
    }
    if TYPE_ID!(unsigned short) != ৩ {
        return ৪;
    }
    
    // Check struct members
    let bf = BF { u: 0, i: 0, b: false };
    if TYPE_ID!(bf.u) != ৪ {
        return ৫;
    }
    if TYPE_ID!(bf.i) != ৫ {
        return ৬;
    }
    if TYPE_ID!(bf.b) != ৬ {
        return ৭;
    }
    
    // Check floating‑point types
    if TYPE_ID!(float) != ৭ {
        return ৮;
    }
    if TYPE_ID!(double) != ৮ {
        return ৯;
    }
    if TYPE_ID!(long double) != ৯ {
        return ১০;
    }
    
    return 0;
}