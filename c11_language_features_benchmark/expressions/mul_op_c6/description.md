**Test name**: mul_op_c6

**Description:**
Validates C11 §6.5.5 multiplicative operator semantics for integer `/` and `%`. The test checks that
integer division truncates the algebraic quotient toward zero and that, when the quotient is
representable, `(a/b)*b + a%b` equals `a` for representative combinations of operand signs.
