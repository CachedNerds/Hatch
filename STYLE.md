<h1 align="center">Rust Style Guide</h1>

*Please write good Rust.*

## Table of Contents

1. [Unsafe](#unsafe)
2. [Whitespace](#whitespace)
3. [Dereferences](#dereferences)


## Unsafe

No use of the `unsafe` keyword is allowed. This guideline is statically enforced.

## Whitespace

All curly brace blocks will have at least one line return between the opening brace and the closing brace. The opening 
curly brace of keyword blocks (e.g. `impl`) will be placed on the same line as the keyword. 

##### Bad
```rust
impl Project { pub fn new() -> Project { ... } }
```

##### Good
```rust
impl Project { 
  pub fn new() -> Project {
    ...
  }
}
```

All `else` and `else if` blocks will begin on the same line as the previous closing curly brace.

##### Bad
```rust
if arg.is_present("bin") { ... }
else if arg.is_present("static") { ... }
else { ... }
```

##### Good
```rust
if arg.is_present("bin") {
  ...
} else if arg.is_present("static") {
  ...
} else {
  ...
}
```

## Dereferences

When `y` is a reference, the asterisk operator used to _dereference_ `y` should
contain no whitespace between the operator and the reference.

##### Bad

```rust
match * y {
  ...
}
```

##### Good

```rust
match *y {
  ...
}
```
