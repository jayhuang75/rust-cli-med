[![Actions Status](https://github.com/jayhuang75/rust-cli-med/workflows/ci/badge.svg)](https://github.com/jayhuang75/rust-cli-med/actions) [![codecov](https://codecov.io/gh/jayhuang75/rust-cli-med/branch/main/graph/badge.svg?token=Z1LMSs2tQC)](https://codecov.io/gh/jayhuang75/rust-cli-med)

# M.E.D. (Mask, Encrypt, Decrypt) - a RUST powered CLI tool for CSV/JSON files

## Background & Motivation

Sometimes in the Enterprise Level, we need a simple enough CLI tool with auditable capability for Data Masking/Encyption/Decryption for CSV/JSON files.

## Key Features

1. Rust powered performance.
2. Provide Masking, and Encyption/Decryption capabilities.
3. Auditable with build-in SQLite powered Audit table.

## Extendability

There are 2 main crates in this package.

1. [MED_CLI](med_cli/README.md) - the CLI interface for the med binary.
2. [MED_CORE](med_core/README.md) - the core engineer to execution the CSV/JSON files Masking, Encryption and Decryption, which you can use in your own usecase/project/context implementation.

## Documentation (TODO)

- [ ] Application Architecture Design
- [ ] Rust code level documentation

## Contributions

I gladly accept contributions via GitHub pull requests. However please complete the [template](.github/workflows/PULL_REQUEST_TEMPLATE.md) before the PR.

Thank you!
