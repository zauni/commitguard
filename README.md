# commitguard

> A commitlint implementation in Rust.

It's a project to learn Rust, so don't expect production ready code. If anybody has suggestions to make it better, please feel free to open an issue or a PR!

Features:

- 🎚️ Configurable
- 🚀 Blazing fast because
- 🦀 Written in Rust

## Installation

```sh
cargo install commitguard
```

## Usage

> Not quite yet, but it will be something like this:

```sh
echo "foo" | commitguard
```

## Todos/Ideas:

1. Configuration system
2. Implement more rules (see [commitlint rules](https://github.com/conventional-changelog/commitlint/tree/master/%40commitlint/rules/src) or [gitlint](https://jorisroovers.com/gitlint/latest/rules/builtin_rules/))
3. Implement unit tests for rules
4. Allow multiple lines in footer and parse references (like original commitlint)
5. Use and parse CLI args (probably with [Clap](https://docs.rs/clap/latest/clap/index.html))
6. Publish on npm and publish binaries for different platforms (maybe also in package managers like Homebrew)
7. Dogfood it to lint our own commit messages
8. Add website (probably Github pages) and add links to rule details
9. Add devcontainer for easier getting started in VSCode
10. Benchmark against original [commitlint](https://github.com/conventional-changelog/commitlint)
11. Migration docs from original commitlint to commitguard
12. Allow 3rd party rules? Maybe WASM?
13. Allow custom parsing?
