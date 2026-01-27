**Test name**: prefix_inc_c1

**Description:**
Validates C11 §6.5.3.1 prefix increment semantics. The test checks that `++E` increments the value of
its operand and that the result of the expression is the new value after incrementation, consistent
with the equivalence between `++E` and `(E += 1)`.
