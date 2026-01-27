**Test name**: tags_1

**Description:**
This code validates C11 §6.7.2.3 (Tags) and verifies that a structure tag introduces a type name that
can be used to declare pointers to the same structure type within its own definition, enabling self-referential structures.
In particular, this test checks the following:

- The tag struct tnode is visible within its own definition
- Pointers to struct tnode are legal inside the struct
- Objects and pointers declared later use the same tagged type
- Member access expressions behave as specified:
