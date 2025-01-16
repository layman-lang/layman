# Types and Type System Test Suite

This directory tests the core type system of Layman, ensuring that the language's strong static typing rules are enforced correctly.

## Concepts Covered
- **Primitive Types**: Verification of Number, String, Boolean, and Void types.
- **Type Inference**: Testing the compiler's ability to deduce types when they aren't explicitly stated (e.g., `the variable x is 5` implies `x` is a Number).
- **Type Compatibility**: Checking rules for assignment and function arguments (e.g., ensuring a String is not accepted where a Number is required).
- **Custom Types**: Interaction with user-defined structs or classes.
