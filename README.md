# Enterprise Rust Template: `rust-arena`

A minimalist, high-performance, locally verifiable Rust template built around strict separation of concerns, designed for enterprise readiness from the very first commit.

*This template currently includes a production-grade implementation of the Gauss-Newton numerical optimization algorithm using `nalgebra` as a demonstration of the workspace structure.*

## Development Philosophy

This template strictly adheres to a decoupled development philosophy. It assumes you do not want an IDE that absorbs every part of the lifecycle. Instead, it provides a small set of independent, composable tools, each with a single responsibility, connected through the filesystem, the command line, Cargo, and CI.

* **Neovim** → Write Code (Editor should remain an editor)
* **Bacon** → Continuous local verification (Event-driven)
* **Cargo / Taskfile** → Declarative intent and strict authority
* **rust-lldb / rr** → Deliberate, explicit runtime debugging
* **betterhook** → Pre-commit repository invariant enforcement
* **act** → Local CI verification

## Core Configuration & Tooling

### 1. High-Performance Testing (`cargo-nextest`)
Standard `cargo test` is replaced entirely with [Nextest](https://nexte.st/). Nextest provides isolated, parallelized, and significantly faster execution for your unit and integration tests.
* **Local:** `task test` invokes `cargo nextest run`.
* **Bacon:** `bacon.toml` overrides the default test job, so running `bacon test` seamlessly triggers Nextest with its specialized failure analyzers.
* **CI:** Installs the native Nextest binary in under a second using `taiki-e/install-action@v2.87.15`.

### 2. Tmpfs Build Optimization (`.cargo/config.toml`)
To eliminate SSD wear and drastically improve build speeds, the local target directory is explicitly routed to `/tmp/cargo-targets/rust-arena` (which resides in RAM). The `Taskfile.yaml` debug hooks correctly point to this external directory when launching `rust-lldb` or `rr`.

### 3. Bulletproof CI Pipeline (`act` & GitHub Actions)
The `.github/workflows/ci.yml` is engineered to be 100% reproducible both on GitHub and locally via `task ci` (which triggers `act`):
* **Deterministic Toolchains:** Installs the Nightly compiler for formatting and sets the Stable compiler as the default for all other checks.
* **Combined Action Fetch:** Uses `taiki-e/install-action@v2.87.15` to install multiple tools simultaneously. The version is hardcoded to prevent local `act` caching errors on floating tags.
* **Bypassing Rate Limits:** `betterleaks` is installed by explicitly fetching the `v1.8.1` `linux_x64.tar.gz` binary directly via `wget`, entirely avoiding GitHub API rate limits that silently break CI pipelines.

### 4. Code Quality & Security
* **Pre-commit Invariants:** `betterhook.toml` ensures that Nightly `rustfmt`, Clippy (`-D warnings`), and `betterleaks` are run locally before a commit is even created.
* **Supply Chain:** `cargo-audit` is used locally (`task security`) and in CI to check the `Cargo.lock` against the RustSec Advisory Database.

### 5. Production Profile Ceiling
The `[profile.release]` in `Cargo.toml` is tuned to the absolute maximum limits of portable Rust optimizations:
* `opt-level = 3` (Max optimizations)
* `lto = "fat"` (Full cross-crate link-time optimization)
* `codegen-units = 1` (Maximum single-threaded optimization boundary)
* `panic = "abort"` (Zero unwinding overhead)
* `strip = "symbols"` (Binary size reduction)

### 6. Example: Gauss-Newton Optimization
The template includes an enterprise-grade example in `src/lib.rs` implementing a non-linear Gauss-Newton update step:
* Uses `nalgebra` for core numerical representation (zero-copy references `&DMatrix`, `&DVector`).
* Uses **Singular Value Decomposition (SVD)** to mathematically trap ill-conditioned and rank-deficient systems deterministically, rather than relying on Cholesky which can silently succeed on positive semi-definite inputs.

## Quick Start Commands

Use `task` to interact with the project declaratively:
* `task check` - Fast AST validation
* `task test` - Run unit and integration tests via `cargo-nextest`
* `task fmt` - Format the codebase (uses Nightly implicitly)
* `task clippy` - Run lints (`-D warnings`)
* `task miri` - Check for Undefined Behavior using Miri
* `task debug` - Start a `rust-lldb` debugging session attached to the tmpfs binary
* `task rr` - Record an execution trace with `rr`
* `task security` - Audit dependencies and scan for secrets locally
* `task release` - Build the highly-optimized production binary
* `task ci` - Reproduce the entire GitHub Actions pipeline locally

Run `bacon` in a background terminal for continuous feedback as you write code in Neovim.
