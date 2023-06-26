# Type based evaluator in Rust
Modelbasierte Softwareentwicklung Projekt
Arithmetic expressions in Rust - Evaluator
## Authors
- [Phil Krause]
- [David Koch]

## Implementation
In this repository we rebuilt the Evaluator which was originally implemented in Haskell for the course "Modellbasierte Softwareentwicklung" in Rust. We represented the Haskell data-types with the help of enums in Rust.

### Expresions
To be able to use pattern matching for our evaluator we implemented all possible expressions in a newly created expression enum. The Operations contain recursive references to the expression type.

### Evaluator
The evaluator uses the before mentioned pattern matching to correctly evaluate all given expressions. The evaluator simplifies the given expression in the manner of short circuit operations.

### Monads
To represent the maybe monad from Haskell we used the option type, implemented in the Rust standard library.
Furthermore we recreated the "Either" Monad from Haskell as needed via our own implementation as an enum.

## Testing
For testing the main function has been used.