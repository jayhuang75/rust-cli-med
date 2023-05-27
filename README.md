[![Actions Status](https://github.com/jayhuang75/rust-cli-med/workflows/ci/badge.svg)](https://github.com/jayhuang75/rust-cli-med/actions) [![codecov](https://codecov.io/gh/jayhuang75/rust-cli-med/branch/main/graph/badge.svg?token=Z1LMSs2tQC)](https://codecov.io/gh/jayhuang75/rust-cli-med)

# M.E.D. (Mask, Encrypt, Decrypt) - a RUST powered CLI tool

## Background & Motivation

Sometimes in the Enterprise Level, we need a simple enough CLI tool with auditable capabilities for Data Masking/Encyption/Decryption.

## Key Features

1. Rust powered performance.
2. Provide Masking, and Encyption/Decryption capabilities.
3. Auditable with build-in SQLite powered Audit table.

### Installation

The binary name for M.E.D. is med.

Archives of precompiled binaries for med are available for [Windows, macOS and Linux](https://github.com/jayhuang75/rust-cli-med/releases). Users of platforms not explicitly mentioned below are advised to download one of these archives.

## Usage

```bash
$ med --help
A simple to use, enterprise ready, rust powered data masking/encryption/decription cli tool

Usage: med <MODE> --file <FILE> [OPTIONS]

Arguments:
  <MODE>
          What mode to run the program in
          Possible values:
          - mask:    Mask the data by *
          - encrypt: Encrypt the data with provided KEY
          - decrypt: Decrypt the data with provided KEY

Options:
  -t, --type <TYPE> type of file we will process, available option [csv, json] [default: csv]
  -k, --key <KEY> key for Encrypt and Decrypt the file.
  -s, --standard <STANDARD> set the Encrypt and Decrypt standard
        Possible values:
          - des64:  DES standard 64
          - aes128: AES standard 128
          - aes192: AES standard 192
          - aes256: AES standard 256
  -f, --file <FILE> file path for the
  -c, --config <CONFIG> Sets a custom config yml path [default: conf.yaml]
  -o, --output <OUTPUT> Sets a file/directory path for output [default: output]
  -d, --debug <DEBUG> Sets debug flag [possible values: true, false]
  -w, --worker <WORKER> Sets work flag
  -h, --help Print help (see a summary with '-h')
  -V, --version Print version
```

### User Guide

#### Configuration

The configuration file can be any given name of [yaml file](demo/conf/conf_json.yaml).

```bash
// example of the conf.yaml
mask_symbols: "#####" # mask symbols
fields: # list of the cols/fields you want to mask 
  - name
  - email
  - phone
```

#### Example of how to

1. All the demo data are available in the package when you download it. And it's all **RANDOMLY** generated. [csv](demo/data/csv/random_data.csv) [json](demo/data/json/generated.json)
2. You only need to point to the root dir for your files. M.E.D. will take care of the rest.

```bash
// mask the csv files in folders
med mask -f demo/data/csv -c demo/conf/conf_csv.yaml -w 3

// mask the json files in folders
med mask -t json -f demo/data/json -c demo/conf/conf_json.yaml -w 3

// encrypt the csv files 
med encrypt -f demo/data/csv -c demo/conf/conf_csv.yaml -w 4 -k YOUR_SECRET -s des64

// decrypt the json files 
med decrypt -t json -f output/demo/data/json -c demo/conf/conf_json.yaml -w 5 -k YOUR_SECRET -s des64

```

#### Audit database (Sqlite)

M.E.D. uses SQLite for the audit capture, mainly ensuring following the Entreprise level Audit base standard, capture, Who, When, Where(which machine), do what, and status, etc.

The metadata and migration are available [here](audit/migrations/20230512195802_audit_sqlite_datastore.up.sql).

The audit db location will be different depending on your OS.

##### location

| Platform  |  Value |    Example      |
| ------------- | ------------- | ------------- |
| Linux  | $HOME/.config/med  | /home/bob/.config/med |
| MacOS  | $HOME/Library/Application Support/med  | /Users/Bob/Library/Application Support/med |
| Windows  | {FOLDERID_RoamingAppData}/med  | C:\Users\Bob\AppData\Roaming\med |

## Roadmap

- [X] csv processor
- [X] json processor
- [ ] publish to crato.io
- [ ] pubhish to more package management

## Documentation

- [ ] Application Architecture Design
- [ ] Rust code level documentation
