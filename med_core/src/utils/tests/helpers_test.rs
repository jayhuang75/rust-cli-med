use walkdir::WalkDir;

use crate::utils::helpers::is_not_hidden;

#[tokio::test]
async fn test_is_not_hidden() {
    let path = "../demo/data/input/format_err/csv";

    let is_ignored = WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_entry(is_not_hidden)
        .count();
    assert_eq!(is_ignored, 4);

    let is_not_ignored = WalkDir::new(path).follow_links(false).into_iter().count();
    assert_eq!(is_not_ignored, 5);
}
