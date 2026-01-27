**Test name**: struct_and_union_3b

**Description:**
Validates C11 §6.5.2.3 member access when a union type is visible at the point of function definition.
The test is a positive version of the standard's "not a valid fragment": instead of defining an anonymous
union locally and passing pointers to its members to a function that cannot name the union type, this test
defines a named union type and passes a pointer to the union object. The function accesses both members
(`u->s1` and `u->s2`) directly, making the union type visible and member access well-typed.
