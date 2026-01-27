**Test name**: array_subscripting

**Description:**
Validates that array subscripting in C11 (§6.5.2.1) is defined in terms of pointer arithmetic,
where the array operand is converted to a pointer to its first element and the subscript index
is scaled by the size of the pointed-to element type. For multidimensional arrays, this ensures
correct scaling by the size of the inner array.
