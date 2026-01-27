**Test name**: size_of_2

**Description:**
Validates C11 §6.5.3.4 (The `sizeof` and `\_Alignof operators`) Example 2. The test computes the number of
elements in an array using the expression `sizeof array / sizeof array[0]` and checks that the result
matches the array's declared length.
