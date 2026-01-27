**Test name**: tags_1b

**Description:**
This code validates C11 §6.7.2.3 (Tags) and verifies that a structure tag introduces a type name that
can be used to declare pointers to the same structure type within its own definition, enabling self-referential structures.
It is a variant of the tags test.
In particular, this test checks that a typedef name introduced before the struct definition can be used inside the definition after the tag is completed.
