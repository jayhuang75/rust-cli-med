use crate::utils::helpers::csv_fields_exist;
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
