## Rust Playground

```shell
cargo run --bin user -q  # run user exe that consumes lib
cargo build # build debug target
cargo build -r # build prod target w/ optimizations
```

#### TODO

- [ ] handling files
- [ ] store
- [ ] orchestrator

#### Ref

- macros:
  - https://stackoverflow.com/questions/64965194/access-field-of-a-generic-type-in-function
  - https://stackoverflow.com/questions/73353782/rust-implement-trait-for-multiple-types
  - https://stackoverflow.com/questions/34270031/how-to-implement-multiple-traits-for-a-struct-without-repeating-methods
- pydantic-like defaults + env var support for structs:
  - https://www.reddit.com/r/rust/comments/141jzey/is_there_a_pydanticbasesettings_equivalent_in_rust/
- generic types:
  - https://stackoverflow.com/questions/30399953/what-is-the-proper-way-to-create-a-new-generic-struct
- disable struct init without ::new()
  - https://stackoverflow.com/questions/53588819/how-to-restrict-the-construction-of-struct
- debug on trait:
  - https://stackoverflow.com/questions/64298436/how-to-get-an-implementation-of-fmtdebug-for-a-vec-of-dyn-trait-objects