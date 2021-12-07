The Rust code definitely needs a big rewrite/refactor. So, if you want to
contribute to the Rust code, create an issue first to discuss it.

On the other hand, it's easier to contribute YAML code for the
[standard library](https://github.com/sayanarijit/yamlfun/tree/main/src/Std).

Steps:

- Add a module in `Std` directory.
- Include it in the [vm](https://github.com/sayanarijit/yamlfun/tree/main/src/vm.rs).
- Run doctest `cargo run --bin yamlfun-doctest src/Std/$your-module.yml`
