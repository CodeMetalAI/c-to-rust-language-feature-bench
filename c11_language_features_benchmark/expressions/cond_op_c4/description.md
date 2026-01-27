**Test name**: cond_op_c4

**Description:**
Validates C11 §6.5.15 conditional operator semantics. The test checks that the first operand is evaluated
before selecting either the second or third operand, that there is a sequence point between the first
operand and the evaluated branch, that only the selected branch is evaluated, and that the result value
is the value of the evaluated branch.
