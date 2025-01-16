# Negative Test Suite

This is arguably the most critical test suite for compiler robustness. It contains code intentionally designed to break rules—syntax errors, type mismatches, and undefined variables. The goal is to verify that the compiler fails **gracefully** and reports the **correct error message** rather than crashing or accepting invalid code.

## Concepts Covered
- **Syntax Errors**: Invalid grammar, missing keywords, or malformed statements.
- **Type Errors**: Attempting to perform operations on incompatible types (e.g., adding a number to a boolean).
- **Scope Errors**: Accessing variables that haven't been declared or are out of scope.
- **Verification**: Unlike other tests, "success" here means the compiler *failed* as expected.
