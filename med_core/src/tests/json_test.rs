use crate::{
    app::{json::json_processor, processor::ProcessRuntime},
    models::enums::{Mode, Standard},
    utils::{crypto::Cypher, error::MedErrorType},
};

const KEY: &str = "123";

#[tokio::test]
async fn test_json_processor_error() {
    // tx_metadata: flume::Sender<Metadata>,
    // files_path: &str,
    // output_path: &str,
    // process_runtime: ProcessRuntime,
    let (tx_metadata, _) = flume::unbounded();
    let process_runtime = ProcessRuntime {
        fields: vec!["name".to_string()],
        cypher: None,
        standard: None,
        mask_symbols: Some("#####".to_string()),
        mode: Mode::MASK,
    };

    let files_path: &str = "";
    let output_path = "";

    match json_processor(
        tx_metadata.clone(),
        files_path,
        output_path,
        process_runtime,
    ) {
        Ok(_) => {}
        Err(err) => {
            assert_eq!(err.error_type, MedErrorType::IoError);
        }
    }

    // drop the channel once it done.
    drop(tx_metadata);
}

#[tokio::test]
async fn test_json_processor_mask() {
    let (tx_metadata, rx_metadata) = flume::unbounded();
    let process_runtime = ProcessRuntime {
        fields: vec!["name".to_string()],
        cypher: None,
        standard: None,
        mask_symbols: Some("#####".to_string()),
        mode: Mode::MASK,
    };
    let files_path: &str = "../demo/data/json/generated.json";
    let output_path = "../output/demo/data/json/mask/generated.json";

    json_processor(
        tx_metadata.clone(),
        files_path,
        output_path,
        process_runtime,
    )
    .unwrap();

    // drop the channel once it done.
    drop(tx_metadata);

    rx_metadata.iter().for_each(|item| {
        assert_eq!(item.total_records, 5);
    });
}

#[tokio::test]
async fn test_json_processor_encrypt() {
    let (tx_metadata, rx_metadata) = flume::unbounded();
    let process_runtime_encrypt = ProcessRuntime {
        fields: vec!["name".to_string()],
        cypher: Some(Cypher::new(KEY)),
        standard: Some(Standard::DES64),
        mask_symbols: None,
        mode: Mode::ENCRYPT,
    };

    let files_path: &str = "../demo/data/json/generated.json";
    let output_path = "../output/demo/data/json/encrypt/generated.json";

    json_processor(
        tx_metadata.clone(),
        files_path,
        output_path,
        process_runtime_encrypt,
    )
    .unwrap();

    // drop the channel once it done.
    drop(tx_metadata);

    rx_metadata.iter().for_each(|item| {
        assert_eq!(item.total_records, 5);
    });
}

#[tokio::test]
async fn test_json_processor_decrypt() {
    let (tx_metadata, rx_metadata) = flume::unbounded();
    let process_runtime = ProcessRuntime {
        fields: vec!["name".to_string()],
        cypher: Some(Cypher::new(KEY)),
        standard: Some(Standard::DES64),
        mask_symbols: Some("#####".to_string()),
        mode: Mode::DECRYPT,
    };
    let output_path: &str = "../demo/data/json/generated.json";
    let files_path = "../output/demo/data/json/encrypt/generated.json";

    json_processor(
        tx_metadata.clone(),
        files_path,
        output_path,
        process_runtime,
    )
    .unwrap();

    // drop the channel once it done.
    drop(tx_metadata);

    rx_metadata.iter().for_each(|item| {
        assert_eq!(item.total_records, 5);
    });
}

#[tokio::test]
async fn test_json_processor_format_err() {
    let (tx_metadata, rx_metadata) = flume::unbounded();
    let process_runtime = ProcessRuntime {
        fields: vec!["name".to_string()],
        cypher: None,
        standard: None,
        mask_symbols: Some("#####".to_string()),
        mode: Mode::MASK,
    };
    let files_path: &str = "../demo/data/format_err/json/format_err.json";
    let output_path = "../output/demo/data/json/encrypt/generated.json";

    json_processor(
        tx_metadata.clone(),
        files_path,
        output_path,
        process_runtime,
    )
    .unwrap();

    // drop the channel once it done.
    drop(tx_metadata);

    rx_metadata.iter().for_each(|item| {
        assert_eq!(item.failed_records, 1);
    });
}