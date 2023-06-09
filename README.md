[![Actions Status](https://github.com/jayhuang75/rust-cli-med/workflows/ci/badge.svg)](https://github.com/jayhuang75/rust-cli-med/actions) [![codecov](https://codecov.io/gh/jayhuang75/rust-cli-med/branch/main/graph/badge.svg?token=Z1LMSs2tQC)](https://codecov.io/gh/jayhuang75/rust-cli-med) 

# M.E.D. (Mask, Encrypt, Decrypt) - a RUST-powered CLI tool for CSV/JSON files

![picture](documents/logo/data-encryption.png)

## Background & Motivation

This is a personal hobby project; based on the observation, sometimes we need a simple enough CLI tool with auditable capability for Data Masking/Encryption/Decryption for CSV/JSON files.

## Key Features

1. Rust powered performance.
2. Provide Masking and Encyption/Decryption capabilities.
3. Auditable with a built-in SQLite-powered Audit table.

## Extendability

There are two central crates in this package.

1. [MED_CLI](med_cli/README.md) - the CLI interface for the med binary.[![Crates.io](https://img.shields.io/crates/v/med_cli)](https://crates.io/crates/med_cli) [![Crates.io](https://img.shields.io/crates/d/med_cli)](https://crates.io/crates/med_cli)
2. [MED_CORE](med_core/README.md) - the core engineer to execution the CSV/JSON files Masking, Encryption, and Decryption, which you can use in your use-case/project/context implementation. [![Crates.io](https://img.shields.io/crates/v/med_core)](https://crates.io/crates/med_core) [![Crates.io](https://img.shields.io/crates/d/med_core)](https://crates.io/crates/med_core)

## Publication

- [Introduction M.E.D.](https://medium.com/dev-genius/introduction-m-e-d-e001cd83a39f)
- [Build a CLI Tool for Data Masking, Encryption, and Decryption With Rust](https://medium.com/better-programming/build-a-cli-tool-for-data-masking-encryption-and-decryption-with-rust-ad36bea27559)
- [Reduce memory footprint by about 600% for M.E.D. — Performance Matters](https://medium.com/gitconnected/reduce-memory-footprint-by-about-600-for-m-e-d-performance-matters-bec407833e7c)

## Installation

### Download from GitHub release (from sources)

The binary name for M.E.D. is med; it depends on the [med_core](../med_core/README.md).

Archives of precompiled binaries for med are available for [Windows, macOS, and Linux](https://github.com/jayhuang75/rust-cli-med/releases). Users of platforms not explicitly mentioned below are advised to download one of these archives.

### Fedora and Centos

```bash
dnf install med
```

### MacOS X86_64 

```bash
brew tap jayhuang75/med
brew install med
```

## Benchmark

```bash
Model Name: MacBook Pro
Processor Name: 6-Core Intel Core i7
Processor Speed: 2.6 GHz
Total Number of Cores: 6
Memory: 16 GB
```

| File Type | Records | File Size | File Counts | Mode | Field num for mask/encrypt| Elapsed Time | Memory Consumption|
| ------------- | ------------- | ------------- | ------------- | ------------- | ------------- | ------------- |------------- |
| CSV | 120,000,000 | 2.8G | 3 | mask | 1 | ~78 seconds (1.3 mins)| ~2 MB |
| CSV | 120,000,000 | 2.8G | 3 | encrypt (DES64) | 1 | ~182 seconds (3 mins)| ~1.9 MB |
| CSV | 120,000,000 | 2.8G | 3 | encrypt (AES128) | 1 | ~221 seconds (3.6 mins)| ~1.9 MB |
| CSV | 20,000,000 | 471M | 2 | mask | 1 | ~7 seconds| ~1.8 MB |
| CSV | 10,000,000 | 236M | 1 | mask | 1 | ~5 seconds| ~1.8 MB |
| JSON | 129,220 | 10G | 129,220 | mask | 1 | ~2200 seconds(36 mins) | ~62 MB |
| JSON | 64,641 | 5.1G | 64,641 | mask | 1 | ~792 seconds(13 mins) | ~30 MB |
| JSON | 23,519 | 1.9G | 23,519 | mask | 1 | ~284 seconds(4.7 mins) | ~18 MB |
| JSON | 23,519 | 1.9G | 23,519 | encrypt | 1 | ~282 seconds(4.4 mins) | ~18 MB |

## Contributions

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification.

Please complete the [template](.github/workflows/PULL_REQUEST_TEMPLATE.md) before the PR.

Contributions of any kind are welcome!

## Show your support

Give a ⭐️ if this project helped you!

## License

Copyright © 2023 [Wei Huang](https://github.com/jayhuang75/)

This project is Licensed under Apache.
