**Test name**: type_def_1

**Description:**
Section 6.7.8 (Type definitions), Example 1 coverage.
Forces the claimed types and compatibility via compile-time checking: `distance` (typedef `MILES`) must behave as an `int` by being passed to an `int`-only function and used in `int` lvalue contexts; `metricp` (typedef `KLICKSP *`) must be a pointer to a function with no parameter specification returning `int` by calling through it both with no arguments and with extra arguments; `x` and `z` (typedef `range`) and `zp` (pointer to `range`) are checked by passing them to functions that require `range` and `range *` and by validating field access through the pointer.
