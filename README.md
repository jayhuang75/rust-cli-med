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
A simple to use, rust powered data masking/encryption/decription cli tool

Usage: masker [MODE] --file <FILE_PATH> <OPTIONS>

Commands [MODE]:
  mask  mask the fields with *
  encrypt encrypt fields with the key
  decrypt encrypt fields with the key

REQUIRED:
  -f --file  this is required which is point to the files directory

OPTIONS available
  -c --config optional default is the conf.yml
  -o --output optional default is /output
  -t --type optional default is csv, [csv, json] are the two optional choice
  -k --key optional, its only for encrypt, and decrypt
  -d --debug optional, default false
  -w --worker optional, worker for processing, default is 2
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
