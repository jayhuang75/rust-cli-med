use crate::{
    app::{
        csv::{csv_fields_exist, csv_processor},
        processor::ProcessRuntime,
    },
    models::enums::{Mode, Standard},
    utils::{crypto::Cypher, error::MedErrorType},
};
use csv::StringRecord;

#[tokio::test]
async fn test_csv_fields_exist() {
    let fields = vec!["name".to_string()];
    let mut headers = StringRecord::new();
    headers.push_field("job_type");
    headers.push_field("name");
    let index = csv_fields_exist(headers, &fields);
    assert_eq!(index[0], 1);
}

#[tokio::test]
async fn test_csv_processor_error() {
    // tx_metadata: flume::Sender<Metadata>,
    // files_path: &str,
    // output_path: &str,
    // process_runtime: ProcessRuntime,
    let (tx_metadata, _) = flume::unbounded();
    let files_path = "";
    let output_path = "";
    let process_runtime = ProcessRuntime {
        fields: vec!["name".to_string()],
        cypher: None,
        standard: None,
        mask_symbols: Some("#####".to_string()),
        mode: Mode::MASK,
    };

    match csv_processor(
        tx_metadata.clone(),
        files_path,
        output_path,
        process_runtime,
    ) {
        Ok(_) => {}
        Err(err) => {
            assert_eq!(err.error_type, MedErrorType::CsvError);
        }
    }

    // drop the channel once it done.
    drop(tx_metadata);
}

#[tokio::test]
async fn test_csv_processor_mask() {
    let (tx_metadata, rx_metadata) = flume::unbounded();
    let process_runtime = ProcessRuntime {
        fields: vec!["name".to_string()],
        cypher: None,
        standard: None,
        mask_symbols: Some("#####".to_string()),
        mode: Mode::MASK,
    };
    let files_path: &str = "../demo/data/input/csv/random_data.csv";
    let output_path = "../demo/data/output/csv/mask/random_data.csv";

    csv_processor(
        tx_metadata.clone(),
        files_path,
        output_path,
        process_runtime,
    )
    .unwrap();

    // drop the channel once it done.
    drop(tx_metadata);

    rx_metadata.iter().for_each(|item| {
        assert_eq!(item.failed_records, 0);
    });
}

#[tokio::test]
async fn test_csv_processor_encrypt() {
    let (tx_metadata, rx_metadata) = flume::unbounded();
    let key = "123";
    let process_runtime = ProcessRuntime {
        fields: vec!["name".to_string()],
        cypher: Some(Cypher::new(key)),
        standard: Some(Standard::DES64),
        mask_symbols: None,
        mode: Mode::ENCRYPT,
    };
    let files_path: &str = "../demo/data/input/csv/random_data.csv";
    let output_path = "../demo/data/output/csv/encrypt/random_data.csv";

    csv_processor(
        tx_metadata.clone(),
        files_path,
        output_path,
        process_runtime,
    )
    .unwrap();

    // drop the channel once it done.
    drop(tx_metadata);

    rx_metadata.iter().for_each(|item| {
        assert_eq!(item.failed_records, 0);
    });
}

#[tokio::test]
async fn test_csv_processor_decrypt() {
    let (tx_metadata, rx_metadata) = flume::unbounded();
    let key = "123";
    let process_runtime = ProcessRuntime {
        fields: vec!["name".to_string()],
        cypher: Some(Cypher::new(key)),
        standard: Some(Standard::DES64),
        mask_symbols: Some("#####".to_string()),
        mode: Mode::DECRYPT,
    };
    let files_path: &str = "../demo/data/output/csv/encrypt/random_data.csv";
    let output_path = "../demo/data/input/csv/random_data.csv";

    csv_processor(
        tx_metadata.clone(),
        files_path,
        output_path,
        process_runtime,
    )
    .unwrap();

    // drop the channel once it done.
    drop(tx_metadata);

    rx_metadata.iter().for_each(|item| {
        assert_eq!(item.failed_records, 0);
    });
}

#[tokio::test]
async fn test_csv_processor_format_err() {
    let (tx_metadata, rx_metadata) = flume::unbounded();
    let process_runtime = ProcessRuntime {
        fields: vec!["name".to_string()],
        cypher: None,
        standard: None,
        mask_symbols: Some("#####".to_string()),
        mode: Mode::MASK,
    };
    let files_path: &str = "../demo/data/input/format_err/csv/format_err.csv";
    let output_path = "../demo/data/output/csv/format_err/processor_err/random_data.csv";

    csv_processor(
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

#[tokio::test]
async fn test_csv_processor_decrypt_error() {
    let (tx_metadata, rx_metadata) = flume::unbounded();
    let key = "123";
    let process_runtime = ProcessRuntime {
        fields: vec!["name".to_string()],
        cypher: Some(Cypher::new(key)),
        standard: Some(Standard::DES64),
        mask_symbols: None,
        mode: Mode::DECRYPT,
    };
    let files_path: &str = "../demo/data/input/format_err/csv/encrypt_err.csv";
    let output_path = "../demo/data/output/csv/format_err/decrypt_err.csv";

    csv_processor(
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
