**Test name**: compound_literal_3

**Description:**
Validates C11 §6.5.2.5 compound literals combined with designated initializers. The test constructs
`struct point` values using `.x`/`.y` designators and passes them to a function by value, demonstrating
that initialization does not depend on member order. It also takes the address of compound literals and
passes pointers to the unnamed struct objects, matching the standard example patterns.
