# rust-cli-masker

## Background & Motivation

Sometimes in the Enterprise Level SDLC, we need a simple enough CLI tool with auditable capabilities for Data Masking/Tokenization/Encyption.

## Key Features

1. Rust powered performance.
2. Provide Masking, Tokenization, and Encyption/Decryption capabilities.
3. Auditable with build-in SQLite powered Audit table.
4. SDK for RESTful API integration.

## How to

```bash
$ masker --help
A simple to use, enterprise ready, rust powered data masking/encryption/decription cli tool

Usage: masker <MODE> --file <FILE> [OPTIONS]

Arguments:
  <MODE>
          What mode to run the program in
          Possible values:
          - mask:    Mask the data by *
          - encrypt: Encrypt the data with provided KEY
          - decrypt: Decrypt the data with provided KEY

Options:
  -t, --type <TYPE>
          type of file we will process, available option [csv, json]
          [default: csv]
  -k, --key <KEY>
          key for Encrypt and Decrypt the file.
  -f, --file <FILE>
          file path for the
  -c, --config <CONFIG>
          Sets a custom config yml path
          [default: conf.yaml]
  -o, --output <OUTPUT>
          Sets a file/directory path for output
          [default: output]
  -d, --debug <DEBUG>
          Sets debug flag
          [possible values: true, false]
  -w, --worker <WORKER>
          Sets work flag
  -h, --help
          Print help (see a summary with '-h')
  -V, --version
          Print version
```

### Installation

TODO

- [ ] Linux
- [ ] Windows
- [ ] MacOS

### User guide

## Documentation

TODO

- [ ] Application Architecture Design
- [ ] Rust code level documentation

## Roadmap

TODO

- [ ] csv
- [ ] json
