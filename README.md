# rust-learning

## learning-menu

- [`basic-types-numbers-{1-11}`](https://zh.practice.rs/basic-types/numbers.html)
- [`basic-types-char-bool-unit-{1-6}`](https://zh.practice.rs/basic-types/char-bool-unit.html)
- [`basic-types-statements-expressions-{0-3}`](https://zh.practice.rs/basic-types/statements-expressions.html)
- [`basic-types-functions-{1-5}`](https://zh.practice.rs/basic-types/functions.html)
- [`ownership-ownership-{1-9}`](https://zh.practice.rs/ownership/ownership.html)
- [`ownership-borrowing-{1-11}`](https://zh.practice.rs/ownership/borrowing.html)
- [`compound-types-string-{1-12}`](https://zh.practice.rs/compound-types/string.html)
- [`compound-types-array-{1-6}`](https://zh.practice.rs/compound-types/array.html)
- [`compound-types-slice-{1-6}`](https://zh.practice.rs/compound-types/slice.html)
- [`compound-types-tuple-{1-6}`](https://zh.practice.rs/compound-types/tuple.html)
- [`compound-types-struct-{1-8}`](https://zh.practice.rs/compound-types/struct.html)
- [`compound-types-enum-{1-6}`](https://zh.practice.rs/compound-types/enum.html)
- [`flow-control-{1-11}`](https://zh.practice.rs/flow-control.html)
- [`pattern-match-match-iflet-{1-9}`](https://zh.practice.rs/pattern-match/match-iflet.html)
- [`pattern-match-patterns-{1-6}`](https://zh.practice.rs/pattern-match/patterns.html)
- [`method-{1-6}`](https://zh.practice.rs/method.html)
- [`generics-traits-generics-{1-7}`](https://zh.practice.rs/generics-traits/generics.html)
- [`generics-traits-const-generics-{1-3}`](https://zh.practice.rs/generics-traits/const-generics.html)
- [`generics-traits-traits-{1-9}`](https://zh.practice.rs/generics-traits/traits.html)
- [`generics-traits-trait-object-{1-5}`](https://zh.practice.rs/generics-traits/trait-object.html)
- [`generics-traits-advanced-traits-{1-5}`](https://zh.practice.rs/generics-traits/advanced-traits.html)
- [`collections-String-{1-7}`](https://zh.practice.rs/collections/String.html)
- [`collections-vector-{1-8}`](https://zh.practice.rs/collections/vector.html)
- [`collections-hashmap-{1-5}`](https://zh.practice.rs/collections/hashmap.html)
- [`type-conversions-as-{1-5}`](https://zh.practice.rs/type-conversions/as.html)
- [`type-conversions-from-into-{1-5}`](https://zh.practice.rs/type-conversions/from-into.html)
- [`type-conversions-others-{1-5}`](https://zh.practice.rs/type-conversions/others.html)
- [`result-panic-panic-{1-2}`](https://zh.practice.rs/result-panic/panic.html)
- [`result-panic-result-{1-6}`](https://zh.practice.rs/result-panic/result.html)
- [`crate-module-crate-{1-6}`](https://zh.practice.rs/crate-module/crate.html)
  1. `cargo new hello-package`
  2. `cargo new --lib hello-package1`
  3. 包名不同，包根位置不同，分别是 `hello-package`, `hello-package1` 和 `src/main.rs`, `src/lib.rs`
  4. `hello-package1`
  5.
    ```bash
    # FILL in the blanks
    .
    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
    │   ├── main.rs
    │   └── lib.rs
    ```
  6.
  ```bash
  # Create a package which contains 
  # 1. three binary crates: `hello-package`, `main1` and `main2`
  # 2. one library crate
  # describe the directory tree below
  .
  ├── Cargo.toml
  ├── Cargo.lock
  ├── src
  │   ├── main.rs
  │   ├── lib.rs
  │   └── bin
  │       └── main1.rs
  │       └── main2.rs
  ├── tests # directory for integrated tests files
  │   └── some_integration_tests.rs
  ├── benches # dir for benchmark files
  │   └── simple_bench.rs
  └── examples # dir for example files
      └── simple_example.rs
  ```
- [`crate-module-module-{1-5}`](https://zh.practice.rs/crate-module/module.html)
  1-5: practice/crate-module/hello-package
- [`crate-module-use-pub-{1-3}`](https://zh.practice.rs/crate-module/use-pub.html)
- [`comments-docs-{1-4}`](https://zh.practice.rs/comments-docs.html)
- [`formatted-output-{1-9}`](https://zh.practice.rs/formatted-output.html)
- [`lifetime-basic-{1-10}`](https://zh.practice.rs/lifetime/basic.html)
- [`lifetime-static-{1-6}`](https://zh.practice.rs/lifetime/static.html)
- [`lifetime-advance-{1-6}`](https://zh.practice.rs/lifetime/advance.html)
