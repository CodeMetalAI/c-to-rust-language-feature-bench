fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // In Rust, arrays don't decay to pointers in the same way as C.
    // `a` is of type `[i32; 3]`, not `*const i32` or `*mut i32`.
    // We'll simulate the type checking with a custom type id function.
    if type_id(&a) != 1 {
        std::process::exit(1);
    }
    if type_id(&a as *const [i32; 3]) != 2 {
        std::process::exit(2);
    }

    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s: [u8; 4] = *b"abc\0";
    // String literals in Rust are of type `&str` or `&[u8; N]` for byte strings.
    // We'll use a byte string literal for comparison.
    if type_id(b"abc") != 3 {
        std::process::exit(8);
    }
    if s[0] != b'a' {
        std::process::exit(9);
    }
    if s[1] != b'b' {
        std::process::exit(10);
    }
    if s[2] != b'c' {
        std::process::exit(11);
    }
    if s[3] != b'\0' {
        std::process::exit(12);
    }

    std::process::exit(0);
}

fn type_id<T>(_: T) -> u8 {
    // Since Rust doesn't have a direct equivalent to _Generic,
    // we use a trait-based approach to simulate type identification.
    trait TypeId {
        fn type_id() -> u8;
    }

    impl TypeId for *const i32 {
        fn type_id() -> u8 {
            1
        }
    }

    impl TypeId for *const [i32; 3] {
        fn type_id() -> u8 {
            2
        }
    }

    impl TypeId for &[u8; 3] {
        fn type_id() -> u8 {
            3
        }
    }

    impl TypeId for [u8; 4] {
        fn type_id() -> u8 {
            4
        }
    }

    // Default case for other types
    impl<T> TypeId for T {
        default fn type_id() -> u8 {
            99
        }
    }

    // Use specialization via the default keyword (requires #![feature(specialization)] or a workaround).
    // Since we can't use specialization in stable Rust, we'll use a different approach with a helper function.
    // Instead, we'll manually match the types we care about.
    // However, we need to handle the specific calls from main.
    // Let's refactor: we'll use a macro to simulate _Generic for the known types.
    // But to keep it simple, we'll implement a separate function for each call.
    // Alternatively, we can use a macro to generate the type_id function.
    // Since the original code only uses specific types, we can hardcode the checks.
    // But to mimic the original behavior, we'll create a macro that works like _Generic for the given types.
    // However, the original _Generic is evaluated at compile time based on the type of the expression.
    // In Rust, we can use a macro to achieve similar compile-time type dispatch.

    // Since we can't easily replicate _Generic in stable Rust without complex traits,
    // we'll implement a simpler version that matches the specific calls in main.
    // We'll use a macro to simulate _Generic for the limited set of types used in the original code.
    // This is a workaround for the lack of _Generic in Rust.
    99 // Placeholder, but we'll replace this with a macro-based solution.
}

// Define a macro to simulate _Generic for the types used in the original code.
macro_rules! TYPE_ID {
    ($expr:expr) => {{
        // Use a trait to get the type id for the expression's type.
        trait TypeIdHelper {
            const ID: u8;
        }

        // Implement for the specific types.
        impl TypeIdHelper for [i32; 3] {
            const ID: u8 = 1;
        }

        impl TypeIdHelper for *const [i32; 3] {
            const ID: u8 = 2;
        }

        impl TypeIdHelper for &'static [u8; 3] {
            const ID: u8 = 3;
        }

        impl TypeIdHelper for [u8; 4] {
            const ID: u8 = 4;
        }

        // Default implementation for other types.
        impl<T> TypeIdHelper for T {
            default const ID: u8 = 99;
        }

        // Use the trait to get the ID.
        <_ as TypeIdHelper>::ID
    }};
}

// However, the above macro uses specialization (default) which is unstable.
// So we cannot use it in stable Rust. Instead, we'll use a different approach.

// We'll create a function that uses pattern matching on the type via any::TypeId.
// But we cannot pattern match on types directly in stable Rust without a trait.
// So we'll use a trait with associated constants.

trait TypeIdTrait {
    const ID: u8;
}

impl TypeIdTrait for [i32; 3] {
    const ID: u8 = 1;
}

impl TypeIdTrait for *const [i32; 3] {
    const ID: u8 = 2;
}

impl TypeIdTrait for &'static [u8; 3] {
    const ID: u8 = 3;
}

impl TypeIdTrait for [u8; 4] {
    const ID: u8 = 4;
}

// For other types, we cannot provide a default implementation without specialization.
// So we'll use a macro that only works for the types we've implemented.
macro_rules! TYPE_ID {
    ($expr:expr) => {{
        // This macro only works for the types we've implemented TypeIdTrait for.
        // For other types, it will fail to compile.
        // We'll use a helper function that returns the ID for the type of the expression.
        fn get_type_id<T: TypeIdTrait>(_: &T) -> u8 {
            T::ID
        }
        get_type_id(&$expr)
    }};
}

// But the original code uses _Generic on expressions, and we need to handle the default case.
// We can use a wrapper that implements TypeIdTrait for all types with a default ID.
// This requires a blanket implementation, which we can do with a marker trait.

trait TypeIdDefault {
    const ID: u8 = 99;
}

impl<T> TypeIdDefault for T {}

// Then modify TypeIdTrait to inherit from TypeIdDefault and override for specific types.
trait TypeIdTrait2: TypeIdDefault {
    // This trait inherits the default ID from TypeIdDefault.
}

impl<T> TypeIdTrait2 for T {}

// Now we can implement TypeIdTrait2 for specific types to override the ID.
impl TypeIdTrait2 for [i32; 3] {
    const ID: u8 = 1;
}

impl TypeIdTrait2 for *const [i32; 3] {
    const ID: u8 = 2;
}

impl TypeIdTrait2 for &'static [u8; 3] {
    const ID: u8 = 3;
}

impl TypeIdTrait2 for [u8; 4] {
    const ID: u8 = 4;
}

// Then the macro becomes:
macro_rules! TYPE_ID {
    ($expr:expr) => {{
        fn get_type_id<T: TypeIdTrait2>(_: &T) -> u8 {
            T::ID
        }
        get_type_id(&$expr)
    }};
}

// Now we can rewrite main using the macro.
fn main_rewritten() {
    let a: [i32; 3] = [10, 20, 30];

    if TYPE_ID!(a) != 1 {
        std::process::exit(1);
    }
    if TYPE_ID!(&a as *const [i32; 3]) != 2 {
        std::process::exit(2);
    }

    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s: [u8; 4] = *b"abc\0";
    if TYPE_ID!(b"abc") != 3 {
        std::process::exit(8);
    }
    if s[0] != b'a' {
        std::process::exit(9);
    }
    if s[1] != b'b' {
        std::process::exit(10);
    }
    if s[2] != b'c' {
        std::process::exit(11);
    }
    if s[3] != b'\0' {
        std::process::exit(12);
    }

    std::process::exit(0);
}

// However, the original code uses _Generic on "abc" which is a string literal.
// In Rust, b"abc" is of type &[u8; 3], which matches our implementation.
// But note: the original code uses const char* for "abc", and const char[4] for s.
// We have implemented for &[u8; 3] (for b"abc") and [u8; 4] for s.

// Let's run main_rewritten instead of the original main.
// But the problem says to output a single main function.
// So we'll replace the entire main with the rewritten version.

// Final main function:
fn main() {
    let a: [i32; 3] = [10, 20, 30];

    if TYPE_ID!(a) != 1 {
        std::process::exit(1);
    }
    if TYPE_ID!(&a as *const [i32; 3]) != 2 {
        std::process::exit(2);
    }

    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s: [u8; 4] = *b"abc\0";
    if TYPE_ID!(b"abc") != 3 {
        std::process::exit(8);
    }
    if s[0] != b'a' {
        std::process::exit(9);
    }
    if s[1] != b'b' {
        std::process::exit(10);
    }
    if s[2] != b'c' {
        std::process::exit(11);
    }
    if s[3] != b'\0' {
        std::process::exit(12);
    }

    std::process::exit(0);
}

// Define the trait and implementations for the macro.
trait TypeIdDefault {
    const ID: u8 = 99;
}

impl<T> TypeIdDefault for T {}

trait TypeIdTrait2: TypeIdDefault {
    // This trait inherits the default ID from TypeIdDefault.
}

impl<T> TypeIdTrait2 for T {}

impl TypeIdTrait2 for [i32; 3] {
    const ID: u8 = 1;
}

impl TypeIdTrait2 for *const [i32; 3] {
    const ID: u8 = 2;
}

impl TypeIdTrait2 for &'static [u8; 3] {
    const ID: u8 = 3;
}

impl TypeIdTrait2 for [u8; 4] {
    const ID: u8 = 4;
}

macro_rules! TYPE_ID {
    ($expr:expr) => {{
        fn get_type_id<T: TypeIdTrait2>(_: &T) -> u8 {
            T::ID
        }
        get_type_id(&$expr)
    }};
}