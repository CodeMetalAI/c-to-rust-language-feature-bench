**Test name**: compound_literal_8

**Description:**
Validates C11 §6.5.2.5 (Compound literals) Example 8. The test mirrors the standard example showing that
a compound literal creates a single unnamed object for the scope in which it appears. Re-evaluating the
same compound literal expression in the same scope yields the same address, and the object’s contents are
updated according to the initializer. The test checks that the pointer value is stable across iterations
and that the stored member value reflects the final initialization.
