# ⚡ decompress-yt | High-Performance Rust Archive Extractor

[![Rust](https://img.shields.io/badge/language-Rust-f94023.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Platform Target](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-blue)](https://doc.rust-lang.org/stable/std/cfg/)

A production-grade, memory-safe Command Line Interface (CLI) decompression utility engineered in Rust. This tool efficiently parses, validates, and extracts zip archives while maintaining strict security mitigations and cross-platform fidelity.

Developed as a deep dive into systems programming, low-level I/O serialization, and conditional compilation.

---

## 🚀 Key Engineering Features

- **🔒 Zip Slip Vulnerability Mitigation:** Uses structural path sanitization via `.enclosed_name()` to block directory traversal attacks—preventing malicious archives from writing files outside the target destination.
- **🐧 Native Unix Permission Preservation:** Leverages conditional compilation (`#[cfg(unix)]`) and the `PermissionsExt` ecosystem to dynamically detect, decode, and re-apply original Unix file modes and execution bits.
- **📂 Defensive I/O & Directory Reconstruction:** Implements non-destructive parent directory streaming routines (`fs::create_dir_all`) that resolve missing tree paths on-the-fly during extraction loops.
- **📊 Low-Memory I/O Streaming:** Employs optimized standard library buffer copies (`io::copy`) to pipe uncompressed byte streams straight to OS file descriptors without risking high RAM allocation or buffer overflows.

---

## 🏗️ Architecture & Implementation Flow

The application isolates concerns between core archive decompression loops and platform-dependent metadata configuration:

1.  **CLI Boundary Layer:** Evaluates runtime arguments, captures system execution contexts, and handles failures elegantly using standard POSIX exit status signals (`0` for success, `1` for error).
2.  **Archive Parser:** Validates zip local headers and memory-maps files sequentially by index.
3.  **Target Verification:** Validates boundaries and constructs safe execution blocks.
4.  **Platform Integration Layer:** Dispatches target attributes to platform-specific sub-routines cleanly away from the main thread execution loop.

---

## 🔧 Installation & Build

Ensure you have the stable Rust toolchain configured on your system.

```bash
# Clone the repository
git clone [https://github.com/archibong-benjamin/decompress-yt.git](https://github.com/archibong-benjamin/decompress-yt.git)

# Move into workspace
cd decompress-yt

# Build optimized release binary
cargo build --release
```
