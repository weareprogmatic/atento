# Contributing to Atento

First off, thank you for considering contributing to Atento! Your help makes this project better and more useful for sysadmins, IT admins, and MSPs.

This document outlines how to contribute safely, consistently, and in alignment with our dual MIT/Apache-2.0 license.

---

## 1. Code of Conduct

Please follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct) in all interactions.  
Be respectful, helpful, and constructive.

---

## 2. How to Contribute

You can contribute in several ways:

1. **Bug Reports / Feature Requests**
   - Use GitHub Issues: [https://github.com/weareprogmatic/atento/issues](https://github.com/weareprogmatic/atento/issues)
   - Provide a clear description, steps to reproduce, and expected behavior.

2. **Code Contributions**
   - Fork the repo and clone locally:
     ```bash
     git clone https://github.com/weareprogmatic/atento.git
     cd atento
     ```
   - Create a feature branch:
     ```bash
     git checkout -b feature/your-feature-name
     ```
   - Make your changes in the `core/` or `cli/` folders as appropriate.
   - Run tests to ensure nothing breaks:
     ```bash
     cargo test --workspace
     ```
   - Commit changes with clear messages:
     ```bash
     git commit -am "Add feature X to core engine"
     ```
   - Push to your fork and create a pull request against `main`.

3. **Documentation**
   - Improving README, CLI examples, or tutorials is highly encouraged.
   - Use clear, concise language with examples where possible.

---

## 3. Coding Standards

- **Rust formatting:** Run `cargo fmt` before submitting.
- **Linting:** Run `cargo clippy` to catch common issues.
- **Tests:** Add unit or integration tests for any new functionality.
- **Commit Messages:** Keep them short and descriptive (imperative style).

---

## 4. Branching & Releases

- `main` is the stable branch.
- Feature work should be done in separate branches.
- Versioning follows **semantic versioning**.

---

## 5. Licensing

By contributing, you agree that your contributions will be licensed under the dual **MIT OR Apache-2.0** license, consistent with the rest of Atento.

---

## 6. Communication

- Use GitHub Issues and Pull Requests for all discussions.
- Major design discussions can happen in dedicated GitHub Discussions or Slack/Discord channels if available.

---

Thank you for helping make **Atento** better for everyone!

