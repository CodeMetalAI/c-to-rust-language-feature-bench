**Test name**: compound_literals_2

**Description:**
Validates C11 §6.5.2.5 block-scope compound literals. The test assigns a pointer using a compound literal
with a non-constant initializer expression (`*p`). It checks that the pointer is set to the first element
of a new two-element array where the first element equals the previous `*p` value and the second element
defaults to zero. This demonstrates that block-scope compound literals need not use constant expressions
and that the unnamed object has automatic storage duration.
