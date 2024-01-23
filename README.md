# commitlint-rs

> A commitlint implementation in Rust.

It's a project to learn Rust, so don't expect production ready code. If anybody has suggestions to make it better, please feel free to open an issue or a PR!

## Todos/Ideas:

1. Configuration system
2. Implement more rules (see [commitlint rules](https://github.com/conventional-changelog/commitlint/tree/master/%40commitlint/rules/src))
3. Implement unit tests for rules
4. Allow multiple lines in footer and parse references (like original commitlint)
5. Use and parse CLI args (probably with [Clap](https://docs.rs/clap/latest/clap/index.html))
6. Publish on npm
7. Dogfood it to lint our own commit messages
8. Benchmark against original [commitlint](https://github.com/conventional-changelog/commitlint)
9. Migration docs from original commitlint to commitlint-rs
10. Allow 3rd party rules? Maybe WASM?