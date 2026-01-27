**Test name**: function_specifiers_2

**Description:**
Section 6.7.4 (Function specifiers), Example 2 coverage.
Defines functions declared with the `_Noreturn` specifier and exercises only defined behavior by ensuring control flow never returns to the caller. One function is unconditionally non-returning, and the other is conditionally non-returning but invoked only along a path that does not return, matching the requirements of the `_Noreturn` specifier.
