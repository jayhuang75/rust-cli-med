[![Crates.io](https://img.shields.io/crates/v/med_core)](https://crates.io/crates/med_core) [![Actions Status](https://github.com/jayhuang75/rust-cli-med/workflows/ci/badge.svg)](https://github.com/jayhuang75/rust-cli-med/actions) [![codecov](https://codecov.io/gh/jayhuang75/rust-cli-med/branch/main/graph/badge.svg?token=Z1LMSs2tQC)](https://codecov.io/gh/jayhuang75/rust-cli-med) [![Crates.io](https://img.shields.io/crates/d/med_core)](https://crates.io/crates/med_core)

### M.E.D. (Mask, Encrypt, Decrypt) - The Core Engine for Masking, Encryption, and Decryption the CSV/JSON files

The core engine design for the plugin by different use case and context.

Currently its the [CLI interface](../med_cli/README.md). If you have different programming or integration need, you can interact with the Core by is APIs.

### Example

```Rust
let mut new_params = Params::default();
new_params.conf_path = "../demo/conf/conf_json.yaml".to_owned();
new_params.mode = Mode::MASK;
new_params.file_type = FileType::JSON;

let mut new_app = App::new(new_params.clone()).await.unwrap();
new_app.load_job_config().await.unwrap();

let metrics = new_app.process().await.unwrap();
```

### Roadmap

- [X] csv processor
- [X] json processor
- [ ] SDK for the med_core engine
