# The Carlae Programming Language

This document describes the Carlae programming language, which is a minimal, Python-like language. As it is new and under active development, this may be inexact and incomplete. However, the core semantics and features should become apparent.

## Values

| Value | Examples | Notes |
|---|---|---|
| Number | `42`, `3.5` | All numbers are 64-bit floats, no separate ~~int~~ type. |
| String | `"mango"` | Double quotes only, no escape sequences, no triple quotes, no f-strings. |
| Boolean | `True`, `False` | Also behave as `1` and `0` in arithmetic and numeric comparisons as in Python. |
| None | `None` | Represents no value. |
| Function | A name defined with `def`, built-in funcs like `clock` | Can be assigned to variables and passed as arguments. |

Types are dynamic. No type annotations. No exponent notation such as `1e3`.

**Falsey values:** `False`, `0`, `""` and `None`. Everything else is truthy, just as in Python.

## Expressions

| Operation | Syntax |
|---|---|
| Arithmetic | `+`, `-`, `*`, `/` and unary `-` |
| Comparison | `<`, `<=`, `>`, `>=`, `==`, `!=` |
| Logic | `not`, `and`, `or` |
| Grouping | `(expression)` |
| Function call | `name(arg1, arg2)` |

- Arithmetic and logic follow the usual Python precedence rules.
- `and` and `or` short-circuit and return an operand, not necessarily a boolean.
- Strings join with `+` and repeat with `*`: `"ha" * 3`. Repetition accepts `3.0` too, but not `3.5`.
- Division by zero is an error.
- **Comparisons do not chain like Python.** Write `a < b and b < c`, not `a < b < c`. Chains currently evaluate as successive binary operations.

## Statements and layout

Supported statements:
- Assignment `name = value`
- `if` and `else`
- `while`
- `return`
- `print` (behaves per Python 2)

```python
def magnitude(x):
    if x < 0:
        return -x
    return x

n = -2
while n < 2:
    print n, magnitude(n)
    n = n + 1
```

- Use spaces or tabs consistently. Do not mix them.
- Parentheses or a trailing `\` allow continuation across lines.
- `#` starts a comment. Blank lines and comment-only lines are ignored.
- Single-line bodies work: `if True: print 1`. No semicolons.

### Printing

Unlike Python 3, `print` is a **statement**, not a built-in function. There are no `sep` or `end` arguments.

```python
print 1, 2       # prints 1 2 then a newline
print "next",    # trailing comma supress final newline
print            # prints a newline
```

## Functions and scope

- Calls take positional arguments only and must match the parameter count. No defaults, keyword arguments or `*args`.
- Names not yet bound locally are read from globals, not from the caller's locals. Reading a global before assigning the same name locally is allowed, unlike Python. No `global` or `nonlocal`.
- `clock()` is the only built-in function: it returns seconds since the Unix epoch. No import is needed.

## Missing Python features

The following are likely to be added soonᵀᴹ.

- REPL!
- Lists, dictionaries and tuples, with basic indexing.
- Classes and closures, but not `lambda` or decorators.
- for, `elif`, `break`, `continue`.
- More built-ins such as `range` or `len`.
- `%`, `//`, `**`, unary `+`, augmented assignment such as `+=` and unpacking assignment.
