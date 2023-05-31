[![Actions Status](https://github.com/jayhuang75/rust-cli-med/workflows/ci/badge.svg)](https://github.com/jayhuang75/rust-cli-med/actions) [![codecov](https://codecov.io/gh/jayhuang75/rust-cli-med/branch/main/graph/badge.svg?token=Z1LMSs2tQC)](https://codecov.io/gh/jayhuang75/rust-cli-med) 

# M.E.D. (Mask, Encrypt, Decrypt) - a RUST powered CLI tool for CSV/JSON files

## Background & Motivation

This is a personal hobby project; based on the observation, sometimes we need a simple enough CLI tool with auditable capability for Data Masking/Encyption/Decryption for CSV/JSON files.

## Key Features

1. Rust powered performance.
2. Provide Masking, and Encyption/Decryption capabilities.
3. Auditable with build-in SQLite powered Audit table.

## Extendability

There are 2 main crates in this package.

1. [MED_CLI](med_cli/README.md) - the CLI interface for the med binary.[![Crates.io](https://img.shields.io/crates/v/med_cli)](https://crates.io/crates/med_cli) [![Crates.io](https://img.shields.io/crates/d/med_cli)](https://crates.io/crates/med_cli)
2. [MED_CORE](med_core/README.md) - the core engineer to execution the CSV/JSON files Masking, Encryption and Decryption, which you can use in your own usecase/project/context implementation. [![Crates.io](https://img.shields.io/crates/v/med_core)](https://crates.io/crates/med_core) [![Crates.io](https://img.shields.io/crates/d/med_core)](https://crates.io/crates/med_core)

## Documentation (TODO)

- [ ] Application Architecture Design
- [ ] Rust code level documentation

## Contributions

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification.

Please complete the [template](.github/workflows/PULL_REQUEST_TEMPLATE.md) before the PR.

Contributions of any kind welcome!

## Show your support

Give a ⭐️ if this project helped you!

## License

Copyright © 2023 [Wei Huang](https://github.com/jayhuang75/)

This project Licensed under Apache.
