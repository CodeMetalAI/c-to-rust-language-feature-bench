**Test name**: struct_and_union_1

**Description:**
Validates C11 §6.5.2.3 structure/union member access on a function result. If `f` returns a structure/union
and `x` names a member, then `f().x` is a valid postfix expression that evaluates to the member value.
