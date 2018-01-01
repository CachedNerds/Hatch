<h1 align="center">Rust Style Guide</h1>

*Please write good Rust.*

## Table of Contents

1. [Unsafe](#unsafe)
1. [Whitespace](#whitespace)

## Unsafe

No use of the `unsafe` keyword is allowed. This guideline is statically enforced.

## Whitespace

All curly brace blocks will have at least one line return between the opening brace and the closing brace. The opening 
curly brace of keyword blocks (e.g. `impl`) will be placed on the same line as the keyword. 
```rust
// bad
impl Project { pub fn new() -> Project { ... } }

// good
impl Project { 
  pub fn new() -> Project {
    ...
  }
}
```

All `else` and `else if` blocks will begin on the same line as the previous closing curly brace.
```rust
// bad
if (foo) { doFoo(); }
else if arg.is_present("static") { ProjectKind::Library(LibraryKind::Static) } 
else { ProjectKind::Library(LibraryKind::Shared) }

// good
if arg.is_present("bin") {
  ...
} else if arg.is_present("static") {
  ...
} else {
  ...
}
```
