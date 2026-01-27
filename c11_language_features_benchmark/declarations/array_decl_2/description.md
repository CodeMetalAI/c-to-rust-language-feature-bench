**Test name**: array_decl_2

**Description:**
Section 6.7.6.2 (Array declarators), Example 2 coverage.
Exercises the strict distinction between `extern int *x;` (an object that is a pointer, defined elsewhere) and `extern int y[];` (an incomplete array type whose storage is defined elsewhere). The test defines `x` as a pointer to a separate backing array and `y` as an array object, then performs reads/writes that must affect the correct underlying storage. It also forces "array-ness" of `y` by using it in contexts where it decays to a pointer while still remaining a distinct array object, making it easy for a translation to incorrectly treat `y` as a pointer-like variable.
