# Concurrency Test Suite

Layman implements a high-level concurrency model designed to make parallel execution intuitive. This suite validates the language's ability to manage multiple tasks simultaneously without blocking the main execution thread.

## Concepts Covered
- **Background Tasks**: The `background` keyword allows functions or blocks of code to run asynchronously, similar to launching a lightweight thread or coroutine.
- **Synchronization**: The `wait` command ensures that the main program or other tasks pause execution until a specific background task has completed, preventing race conditions.
- **Concurrent Blocks**: The `concurrently` block allows multiple statements to be executed in parallel, automatically waiting for all of them to finish before proceeding. This is useful for batch processing independent operations.
