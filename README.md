# reqwest-timeouts

A test harness for exploring
[`reqwest::ClientBuilder`](https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html)
timeout settings. A Rust CLI drives requests while an Express.js server
simulates slow or stalled responses.

## Timeouts under test

| Timeout     | `ClientBuilder` method | What it controls                                         |
| ----------- | ---------------------- | -------------------------------------------------------- |
| **Total**   | `timeout()`            | End-to-end deadline from connect through response body   |
| **Read**    | `read_timeout()`       | Per-read idle timeout; resets after each successful read |
| **Connect** | `connect_timeout()`    | TCP connect phase only                                   |

Other `ClientBuilder` timeouts (`pool_idle_timeout`, TCP keep-alive, HTTP/2
keep-alive) are not easily observable from a test client, so they are excluded.

## How it works

```text
┌────────────┐         ┌──────────────┐
│  Rust CLI  │──HTTP──▶│ Express.js   │
│  (reqwest) │         │ test server  │
└────────────┘         └──────────────┘
```

The **Express server** exposes endpoints whose latency is controlled via query
parameters:

- `/delay?seconds=N` — waits _N_ seconds before responding (tests total
  timeout).
- `/slow-body?chunk_delay=N&chunks=M` — sends _M_ chunks with _N_-second pauses
  (tests read timeout).
- `/health` — instant response for sanity checks and connect-timeout success.

The **Rust CLI** (built with `clap`) has one subcommand per timeout. Each
subcommand configures a `ClientBuilder`, makes a request, and prints `SUCCESS`
or `FAILED` with elapsed time.

## Quick start

```bash
npm install        # install Express
cargo build        # build the CLI

npm start &        # start the server on :3000

# success: 2 s timeout, 1 s server delay
cargo run -q -- timeout --timeout 2 --server-delay 1

# failure: 1 s timeout, 3 s server delay
cargo run -q -- timeout --timeout 1 --server-delay 3
```

See [AGENTS.md](AGENTS.md) for the full test plan covering every scenario.

## Formatting

```bash
npm run fmt          # auto-format JS, Markdown (prettier) + Rust (cargo fmt)
npm run fmt:check    # check only — used in CI
```
