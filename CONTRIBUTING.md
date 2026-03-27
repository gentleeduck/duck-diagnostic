# Contributing to gentleduck/diagnostic

Thank you for considering contributing to **gentleduck/diagnostic**.
We welcome all kinds of contributions - bug reports, documentation improvements, feature requests, and code.

---

## Code of Conduct

By participating in this project, you agree to uphold our [Code of Conduct](./CODE_OF_CONDUCT.md).
Please treat everyone with respect and kindness.

---

## Getting Started

### 1. Fork & Clone

```bash
git clone https://github.com/gentleduck/duck-diagnostic.git
cd duck-diagnostic
```

### 2. Build

```bash
cargo build
```

### 3. Run Tests

```bash
cargo test
```

### 4. Run Examples

```bash
cargo run --example demo
```

---

## Development Workflow

1. **Branching**

   * Create a new branch from `master`.
   * Use a descriptive name, e.g. `fix/underline-offset`, `feat/multi-line-spans`, `docs/readme-update`.

   ```bash
   git checkout -b feat/my-feature
   ```

2. **Coding Standards**

   * Run `cargo fmt` before committing.
   * Run `cargo clippy` and fix any warnings.
   * Write clear, self-documenting code.

3. **Commit Messages**

   Follow [Conventional Commits](https://www.conventionalcommits.org/):

   ```
   feat: add multi-line span support
   fix: correct underline alignment in plain text mode
   docs: update api reference in readme
   ```

4. **Testing**

   * Write tests for new functionality.
   * Run all tests before pushing:

     ```bash
     cargo test
     ```

---

## Submitting a Pull Request

1. Push your branch:

   ```bash
   git push origin feat/my-feature
   ```

2. Open a Pull Request against the `master` branch.

3. Fill out the PR template with:

   * A clear description of your changes
   * Any related issues (`Closes #123`)
   * How you tested it

---

## Reporting Issues

If you find a bug, please [open an issue](https://github.com/gentleduck/duck-diagnostic/issues) with:

* Steps to reproduce
* Expected behavior
* Actual behavior
* Rust version (`rustc --version`)

---

## Ways to Contribute

* **Code**: Bug fixes, features, optimizations
* **Docs**: README improvements, examples, guides
* **Testing**: More test coverage, edge cases
* **Community**: Helping others in discussions

---

## License

By contributing, you agree that your contributions will be licensed under the project's [MIT License](./LICENSE).
