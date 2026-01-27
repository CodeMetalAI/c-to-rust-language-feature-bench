**Test name**: compound_literal_7

**Description:**
Validates C11 §6.5.2.5 (Compound literals) Example 7. The test constructs a circularly linked object
using a named `struct int_list` instance whose `cdr` points to itself, and passes it to `eval`. This
demonstrates the intended contrast: compound literals are unnamed, so a single compound literal cannot
directly express a self-referential object, whereas a named object can.
