**Test name**: struct_and_union_2

**Description:**
Validates C11 §6.5.2.3 rules for structure member access qualification. The test checks that selecting a
member with `.` yields the member type with (1) any qualifiers declared on the member itself and (2) any
qualifiers inherited from the base structure object (`const`/`volatile`). Compilation succeeds only if the
member expressions have the qualifier combinations shown in the standard example.
