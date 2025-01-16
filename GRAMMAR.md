# Layman Grammar Specification

This document provides a comprehensive specification of the Layman programming language grammar. It reflects the current implementation where code is written in **pure English**, minimizing special characters and punctuation.

## 1. Lexical Structure

Layman uses a natural language lexer. The code is tokenized into words, numbers, strings, and standard punctuation.

### Comments
- **Line Comments**: Start with `#` and continue to the end of the line.

### Identifiers
- Variable and function names are typically lower_case or snake_case identifiers.
- Multi-word names in variable declarations (e.g., `user name`) are treated as distinct words but generally parsed as single identifiers in practice.

### Literals
- **Numbers**: `123`, `3.14`, `-5`
- **Strings**: `"Hello world"`, `'Single quoted'`, `"""Multi-line strings"""`
- **Booleans**: `true`, `false`

### Keywords
Key reserved words include:
`define`, `function`, `variable`, `if`, `then`, `else`, `while`, `for`, `each`, `in`, `return`, `call`, `with`, `is`, `not`, `and`, `or`, `plus`, `minus`, `times`, `divided by`, `import`, `expect`, `test`, `describe`.

---

## 2. Program Structure

A Layman program consists of a sequence of **statements**.

```ebnf
Program ::= Statement*
```

---

## 3. Statements

Statements are the building blocks of execution.

### Variable Declaration
```
the variable <name> is <expression>
define variable <name> as <expression>
```
*Example:* `the variable count is 10`

### Constant Declaration
```
the constant <name> is <expression>
```
*Example:* `the constant pi is 3.14`

### Assignment
```
set <name> to <expression>
<name> is <expression>
```
*Example:* `set count to 20`

### Function Definition
```
define function <name> that takes <param1> and <param2> ... and returns <Type>
  <body statements>
```
*Example:*
```
define function add that takes a and b and returns Number
  return a plus b
```

### Class Definition
```
define class <Name> with
  property <propName> using <Type>
  method <methodName> that takes ... and returns <Type>
    <body>
```

### Control Flow

#### If/Else
```
if <condition> then
  <statements>
else if <condition> then
  <statements>
else
  <statements>
end if
```

#### Loop (For Each)
```
for each <item> in <collection> do
  <statements>
end for
```

#### Loop (While)
```
while <condition> do
  <statements>
end while
```

#### Repeat
```
repeat <count> times
  <statements>
end repeat
```

---

## 4. Expressions

Expressions evaluate to a value.

### Function Calls (Pure English)
Layman strictly enforces English syntax for calls. Parentheses `()` are **not** used for invocation.

**Syntax:**
```
call <function_name> with <arg1>, <arg2>...
call <method_name> on <object> with <arg1>...
```

*Examples:*
- `call print with "Hello"`
- `call assert_eq with result, 10`
- `call add on list with item`

### Property Access
Properties can be accessed using dot notation or natural language possessives (in future).
Currently supported:
- `<object>.<property>`

*Special Properties:*
- `list.size`, `list.length`, `list.count` -> Number
- `dictionary.size` -> Number

### Operations
Layman supports both symbols and English keywords for operations.

| Operation | Keyword | Symbol |
|-----------|---------|--------|
| Add | `plus` | `+` |
| Subtract | `minus` | `-` |
| Multiply | `times` | `*` |
| Divide | `divided by` | `/` |
| Modulo | `modulo` | `%` |
| Equals | `is`, `equals` | `==`, `=` |
| Not Equal | `is not` | `!=` |
| Greater | `greater than` | `>` |
| Less | `less than` | `<` |
| And | `and` | `&&` |
| Or | `or` | `||` |
| Not | `not` | `!` |

*Example:* `if x is not 10 then...`

---

## 5. Concurrency

Layman supports high-level concurrency primitives.

### Background Tasks
Run a block or function asynchronously.
```
background
  <statements>
end background
```

### Synchronization
Wait for all background tasks to complete.
```
wait
```

### Concurrent Blocks
Execute multiple statements in parallel and wait for all to finish.
```
concurrently
  <stmt1>
  <stmt2>
end concurrently
```

---

## 6. Error Handling

### Throw
Raise an exception.
```
throw <expression>
```
*Example:* `throw "Invalid input"`

### Try/Catch
Handle exceptions.
```
try
  <statements>
catch error
  <statements>
end try
```

---

## 7. Testing Framework

Built-in testing keywords facilitate test-driven development.

```
describe "Feature Name"
  test "Scenario 1"
    <statements>
  end test
end describe
```

Assertions are performed using standard function calls like `call assert_eq with actual, expected`.

---

## 8. Data Structures

### Lists
- **Literal**: `[1, 2, 3]`
- **Access**: `list[0]`
- **Properties**: `list.size`

### Dictionaries
- **Literal**: `{ "key": "value" }`
- **Access**: `dict["key"]`
- **Properties**: `dict.size`

---

## 9. Types

Layman is statically typed but supports inference.

- `Number`
- `String` / `Text`
- `Bool` / `Boolean`
- `Void`
- `List<Type>`
- `Dictionary<KeyType, ValueType>`
- `Any` (dynamic fallback)

---
