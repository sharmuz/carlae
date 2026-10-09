# Carlae TODO

## Next

- ~~`Parser::synchronize()` for syntax-error handling per Nystrom 6.3. Never execute a partial parse.~~
- ~~Add 3 integration tests:~~
  - ~~Test syntax errors returning correct diagnostic without panic.~~
  - ~~Test return returns out of nested `if` or `while`.~~
  - ~~Test `and` and `or` skip a failing right operand.~~
- Start the bytecode VM! Can share script tests with the tree-walker.

## Deferred - not VM blockers

- Make scanning linear and UTF-8-safe before benchmarking.
- Reject newlines in strings or fix line counting.
- Reject top-level `return`.
- Reject duplicate parameters.
- Guard string repetition against overflow.
- Reject or support Python-style chained comparisons.
- Reduce repeated numeric/boolean cases, construct lookup errors lazily.

## Future features

### Most wanted

- Closures
    - Improve scope rules and copying globals.
- Classes
- Lists

### Planned

- REPL
- Dictionaries
- Tuples
- `for` loops
- `print()` built-in
- Integers

## Experiments

- Garbage collection: tracing vs reference counting
- Async: VM suspension/resumption
- Concurrency: threads, tasks and shared state
- Value representations: enums, tagged values and NaN boxing
- Bytecode optimisation: constant folding and specialised instructions
