**Test name**: size_of_1

**Description:**
Validates C11 §6.5.3.4 (The sizeof and \_Alignof operators) Example 1 by using `sizeof *dp` to request
the correct number of bytes from an allocator that returns `void *`, converting the result to `double *`,
and using the allocated object. This exercises the common `alloc(sizeof *ptr)` idiom.
