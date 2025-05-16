# FIPS Hasher

A Rust-based CLI tool and microservice for secure file hashing and auditing. Designed to demonstrate hands-on experience with Rust, asynchronous programming, CLI UX, and secure coding practices (FIPS-compliant primitives).

## Features

-  **Hash Files/Directories** using FIPS-compliant SHA-256
-  **Optional HMAC** support with secret key
-  **Generate and verify reports** for auditing
-  **Benchmarking** mode for hash speed analysis
-  **Optional HTTP API** server with Axum
- ️ **Concurrent processing** using async + `tokio`

---

## Installation

```bash
git clone https://github.com/yourname/fips-hasher.git
cd fips-hasher
cargo build --release