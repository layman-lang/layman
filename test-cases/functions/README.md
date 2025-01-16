# Functions Test Suite

Functions are the primary building blocks of reusable logic in Layman. This suite comprehensively tests the parser and runtime support for defining and calling functions in various configurations.

## Concepts Covered
- **Function Definition**: The `define function` syntax, including parameter lists (`that takes inputs`) and return type declarations (`returns Type`).
- **Invocation**: The natural language syntax for calling functions (e.g., `call my_func with arg1, arg2`).
- **Scope**: Verifying that variables defined inside a function are local to it and do not leak into the global scope.
- **Recursion**: Tests for self-referential function calls to ensure the stack is managed correctly.
