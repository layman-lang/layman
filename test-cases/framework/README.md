# Framework and Utilities

This directory differs from others as it contains **utility code** rather than direct compiler tests. It serves as a library of helper functions and assertions used by other test suites to verify their results.

## Key Components
- **Assertions**: specialized functions like `assert_eq` or `expect` that are used to validate test outcomes. These functions typically throw an error if the actual value does not match the expected value.
- **Test Runners**: Any shared logic responsible for setting up or tearing down test environments.
- **Common Helpers**: Reusable code snippets (e.g., setup routines) that reduce duplication across test files.
