# Contributing

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)
- [Node.js](https://nodejs.org/) ≥ 18

## Setup

```bash
npm install && cargo build
```

## Formatting

Run after every change:

```bash
npm run fmt          # auto-format JS, Markdown, YAML (Prettier) + Rust (cargo fmt)
npm run fmt:check    # check only — used in CI
```

- **Prettier** handles `.js`, `.md`, `.yml`, `.json` files.
- **`cargo fmt`** handles Rust source files.
- Both are run together via `npm run fmt`.

## Testing

For non-formatting code changes, run the test scenarios in
[AGENTS.md](AGENTS.md#test-scenarios) to verify nothing is broken.
