use crate::utils::{crypto::Cypher, enums::Standard};

#[tokio::test]
async fn test_crypto_data() {
    let crypto = Cypher::new(&"magickey".to_string());
    let data = crypto.encrypt("http://magiclen.org", &Standard::AES256).unwrap();
    assert_eq!("DS/2U8royDnJDiNY2ps3f6ZoTbpZo8ZtUGYLGEjwLDQ=", data);
}
