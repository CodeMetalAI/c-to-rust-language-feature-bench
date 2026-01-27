**Test name**: compound_literal_6

**Description:**
Validates C11 §6.5.2.5 (Compound literals) Example 6. The standard permits const-qualified compound
literals to be placed in read-only memory and possibly share storage with identical string literals.
This test constructs `(const char[]){"abc"}` and `"abc"`, verifies that both have the expected contents,
and treats pointer equality as implementation-defined (it may be either true or false).
