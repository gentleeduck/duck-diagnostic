# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.1](https://github.com/gentleeduck/duck-diag/releases/tag/v0.8.1) - 2026-05-05

### Added

- 0.7.0 — smart printer for multi-file engines
- 0.6.0 — source-less compact rendering + internal style helpers
- 0.5.0 — derive Deserialize for Severity, LabelStyle, Applicability
- [**breaking**] 0.4.0 — Span.file is Arc<str>
- 0.3.2 — derive Deserialize for Span
- 0.3.0 — rustc-style diff suggestions
- 0.2.0 — multi-file, suggestions, JSON, Bug severity, Unicode-width

### Fixed

- *(ci)* quote release-plz job names (yaml colon parse)
- *(formatter)* stack same-line carets + add gap between diagnostics

### Other

- 0.8.1 + standardize logo
- manual-only release + CHANGELOG
- drop CodeQL (Rust-only), advisory-only audit
- [**breaking**] rename to duck-diag + sync ci scaffolding from duck-mc
- 0.7.1 — rustdoc on public API + README refactor
- add big showcase example
- add diff_suggestions example
- update readme with logo, badges, and full API docs
- add github templates: issue templates, PR template, funding
- add community docs: security policy, code of conduct, contributing guide, support
- add MIT license
- update Cargo.toml for gentleduck org
- add readme
- add integration tests
- add usage examples: compiler, sql engine, config linter, api validator, demo
- add DiagnosticEngine with emit, extend, getters, and summary rendering
- add diagnostic formatter with colored and plain text output
- add core types: Severity, DiagnosticCode trait, Span, Label, Diagnostic
- init project setup
