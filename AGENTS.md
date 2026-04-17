# AGENTS.md

## Project overview

A test harness for exploring `reqwest::ClientBuilder` timeout behavior.

| Component   | Language       | Entry point   |
| ----------- | -------------- | ------------- |
| CLI client  | Rust           | `src/main.rs` |
| Test server | Node / Express | `server.js`   |

## Prerequisites

- Rust toolchain (edition 2024)
- Node.js ≥ 18

## Build

```bash
npm install && cargo build
```

## After every change

- Run `npm run fmt` to format all code (Prettier for JS/MD/YAML, `cargo fmt` for
  Rust).
- For non-formatting code changes, also run the test scenarios below.

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup and formatting details.

## CLI subcommands

| Subcommand        | Tests                              | Key flags                                                     |
| ----------------- | ---------------------------------- | ------------------------------------------------------------- |
| `timeout`         | `ClientBuilder::timeout()`         | `--timeout <secs>` `--server-delay <secs>`                    |
| `read-timeout`    | `ClientBuilder::read_timeout()`    | `--read-timeout <secs>` `--chunk-delay <secs>` `--chunks <n>` |
| `connect-timeout` | `ClientBuilder::connect_timeout()` | `--connect-timeout <secs>` `--host <url>`                     |

All subcommands default to `http://localhost:3000`.

## Server endpoints

| Route            | Purpose                  | Query params                              |
| ---------------- | ------------------------ | ----------------------------------------- |
| `GET /health`    | Health check             | —                                         |
| `GET /delay`     | Delayed full response    | `seconds`                                 |
| `GET /slow-body` | Chunked body with pauses | `chunk_delay` (seconds), `chunks` (count) |

## Test scenarios

Start the server first: `npm start`

### `timeout`

- **Expect SUCCESS:** `cargo run -q -- timeout --timeout 2 --server-delay 1`
- **Expect FAILED:** `cargo run -q -- timeout --timeout 1 --server-delay 3`

### `read-timeout`

- **Expect SUCCESS:**
  `cargo run -q -- read-timeout --read-timeout 2 --chunk-delay 1 --chunks 3`
- **Expect FAILED:**
  `cargo run -q -- read-timeout --read-timeout 1 --chunk-delay 3 --chunks 3`

### `connect-timeout`

- **Expect SUCCESS:** `cargo run -q -- connect-timeout --connect-timeout 5`
- **Expect FAILED:**
  `cargo run -q -- connect-timeout --connect-timeout 2 --host http://192.0.2.1:1234`
  - `192.0.2.1` is TEST-NET-1 (RFC 5737); behavior may vary on VPNs.

### Quick smoke test

```bash
cargo run -q -- timeout --timeout 2 --server-delay 1 \
  && cargo run -q -- timeout --timeout 1 --server-delay 3 \
  && cargo run -q -- read-timeout --read-timeout 2 --chunk-delay 1 --chunks 3 \
  && cargo run -q -- read-timeout --read-timeout 1 --chunk-delay 3 --chunks 3 \
  && cargo run -q -- connect-timeout --connect-timeout 5
```

Tests 2 and 4 print `FAILED` (expected timeout); the rest print `SUCCESS`.
