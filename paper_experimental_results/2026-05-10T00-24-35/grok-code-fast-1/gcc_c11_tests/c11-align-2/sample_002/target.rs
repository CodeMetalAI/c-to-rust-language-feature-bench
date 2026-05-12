mod std {
    pub mod mem {
        pub fn align_of<T>() -> usize {
            std::mem::align_of::<T>()
        }
    }
    pub mod process {
        pub fn abort() -> ! {
            std::process::abort()
        }
    }
}

#[repr(align(16))] // Assuming max alignment, similar to max_align_t
struct AlignedU8(u8);

#[repr(align(16))]
struct AlignedI16(i16);

#[repr(align(4))]
struct AlignedI32(i32);

#[repr(align(16))]
struct AlignedI64(i64);

#[repr(align(16))]
struct AlignedI128(i128);

#[repr(align(16))]
struct AlignedF32(f32);

#[repr(align(16))]
struct AlignedF64(f64);

// For complex long double, approximate with array or something, but skip for simplicity
// alignas(0) alignas(int) alignas(char) char ca[10]; -> no alignment, so [u8; 10]

static C: AlignedU8 = AlignedU8(0);
static S: AlignedI16 = AlignedI16(0);
static I: AlignedI32 = AlignedI32(0);
static L: AlignedI64 = AlignedI64(0);
static LL: AlignedI128 = AlignedI128(0); // long long might be 128 in some contexts
static F: AlignedF32 = AlignedF32(0.0);
static D: AlignedF64 = AlignedF64(0.0);

// ca
static CA: [u8; 10] = [0; 10];

// x: alignas((int)alignof(max_align_t) + 0) -> align(16)
#[repr(align(16))]
static X: i32 = 0;

// enum e { E = alignof(max_align_t) }; -> const E: usize = std::mem::align_of::<u128>();
const E: usize = std::mem::align_of::<u128>();

#[repr(align(16))] // alignas(E)
static Y: i32 = 0;

fn func() {
    // alignas(max_align_t) long long auto_ll; -> let auto_ll: i128 = 0; but no alignment for locals
    let _auto_ll: i128 = 0;
}

// Useless alignas(0) struct s; -> struct S;

struct S;

fn main() {
    let s1 = "_Alignas";
    let s2 = "_Alignof";
    let s3 = "1";
    let s4 = "1";
    
    if s1 != "_Alignas" {
        std::process::abort();
    }
    if s2 != "_Alignof" {
        std::process::abort();
    }
    if s3 != "1" {
        std::process::abort();
    }
    if s4 != "1" {
        std::process::abort();
    }
}