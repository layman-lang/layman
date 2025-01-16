# Error Handling Test Suite

Robust error handling is crucial for reliable software. This directory contains tests that verify Layman's mechanisms for detecting, throwing, and recovering from runtime errors.

## Concepts Covered
- **Try/Catch Blocks**: The primary mechanism for safe execution. Tests verify that dangerous code inside a `try` block passes control to the `catch` block upon failure, preventing the program from crashing.
- **Throwing Errors**: The explicit use of the `throw` keyword to generate custom runtime errors. This is often used for validation logic.
- **Error Propagation**: Verifying how errors bubble up from function calls if not caught immediately.
