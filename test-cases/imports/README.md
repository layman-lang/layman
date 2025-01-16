# Imports and Modules Test Suite

This directory tests Layman's module system, which allows code to be organized across multiple files and packages. It ensures that the language can correctly resolve, load, and scope external dependencies.

## Concepts Covered
- **Import Syntax**: The `import` statement in its various forms, such as importing entire modules or specific items.
- **File Resolution**: Verifying that the compiler can locate modules in the local file system using relative paths.
- **Scoping**: Ensuring that imported names do not conflict with local names unless explicitly aliased or qualified.
- **Circular Imports**: Testing the resolver's ability to detect or handle recursive dependency graphs.
