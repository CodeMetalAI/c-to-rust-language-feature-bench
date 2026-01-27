**Test name**: restrict_def_4

**Description:**
Section 6.7.3.1 (restrict), Example 4 coverage.
Exercises the defined "outer-to-inner" flow of restricted pointers across nested blocks by initializing inner-block `restrict` pointers from outer-block `restrict` pointers and using them for disjoint read/write accesses. The test stresses translation by reusing the same underlying arrays through different restricted pointer objects across scopes, while validating that the inner-block computation updates only the intended destination range and never modifies the source array.
