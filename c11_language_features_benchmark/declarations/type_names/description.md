**Test name**: type_names

**Description:**
Section 6.7.7 (Type names) coverage (Example a-h, with (e) using the VM form accepted by C11 compilers).
Exercises identifier-less type names via casts and compound literals to cover: `int`, `int *`, `int *[3]`, `int (*)[3]`, and a variably modified pointer-to-array type name `int (*)[n]` (portable stand-in for `int (*)[*]`). Also exercises `int *()` by assigning a function to a no-prototype function-pointer type returning `int *`, exercises `int (*)(void)` via a pointer to a no-argument function returning `int`, and exercises `int (*const [])(unsigned int, ...)` via an array of const-qualified variadic function pointers.
