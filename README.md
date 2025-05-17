# FIPS Hasher

A Rust-based CLI tool and microservice for secure file hashing and auditing.

What it does:

- Measures and reports file integrity using FIPS-compliant cryptographic hashing (SHA-256, HMAC).

- Helps you audit whether files have been changed or tampered with over time.

- Provides a tool to document and verify file state snapshots (hash reports).

- Supports automation and monitoring through CLI and optional API

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
```

##Testing

Test hashing
Run the hash command on this directory without exclude patterns:

bash
Copy
Edit
cargo run -- hash --path ./test_data
You should see SHA256 hashes printed for all files including .log.

Manually verify hashes if you want (e.g., use sha256sum command on files).

3. Test report generation
Generate a JSON report from the same directory:

bash
Copy
Edit
cargo run -- report --path ./test_data --out report.json
Verify that report.json is created and contains all file paths and hashes.

4. Test verification (matching case)
Now verify the directory against the generated report:

bash
Copy
Edit
cargo run -- verify --path ./test_data --report report.json
It should print “All files match the report.”

5. Test verification (mismatch case)
Modify one of the files (e.g., change content in file1.txt), then run the verify command again:

bash
Copy
Edit
echo "tampered content" > ./test_data/file1.txt
cargo run -- verify --path ./test_data --report report.json
It should print “Mismatch found!”

6. Test benchmarking
Run the benchmark command:

bash
Copy
Edit
cargo run -- benchmark
It should report a time elapsed for hashing current directory (or .).

7. Test server
Start the server:

bash
Copy
Edit
cargo run -- serve --addr 127.0.0.1:3000
Then in another terminal, query the root endpoint:

bash
Copy
Edit
curl http://127.0.0.1:3000/
It should respond with "fips-hasher API".

