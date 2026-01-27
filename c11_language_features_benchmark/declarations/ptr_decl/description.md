**Test name**: ptr_decl

**Description:**
Section 6.7.6.1 (Pointer declarators) coverage.
Stresses the distinction between `const int *` (re-targetable pointer to const value) and `int *const` / `const int_ptr` (const-qualified pointer to modifiable int) by threading both forms through a struct, function calls, and conditional re-targeting. The test confirms the pointer-to-const can be reassigned between objects while the const-qualified pointers continue to designate the original target and remain usable for writes through the pointer.
