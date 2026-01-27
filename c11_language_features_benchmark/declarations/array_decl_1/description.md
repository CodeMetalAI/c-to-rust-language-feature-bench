**Test name**: array_decl_1

**Description:**
Section 6.7.6.2 (Array declarators) coverage, including Example 1.
Declares `float fa[11], *afp[17]` and uses block-scope VLAs whose bounds are integer expressions evaluated on block entry and constrained to be > 0. Validates that the VLA instance size stays fixed for its lifetime via in-lifetime pointer arithmetic, and exercises function-parameter array declarators with outermost `static` and type qualifiers (`float a[static const 11]`, `float *afp[static const 17]`) to require at least the specified number of elements.
