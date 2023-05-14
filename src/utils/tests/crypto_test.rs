use crate::utils::crypto::CryptoData;

#[tokio::test]
async fn test_crypto_data() {
    let crypto = CryptoData::new(&"magickey".to_string());
    let data = crypto.encrypt("http://magiclen.org").unwrap();
    assert_eq!("DS/2U8royDnJDiNY2ps3f6ZoTbpZo8ZtUGYLGEjwLDQ=", data);
}