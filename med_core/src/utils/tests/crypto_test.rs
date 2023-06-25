use crate::models::enums::Standard;
use crate::utils::crypto::Cypher;

#[tokio::test]
async fn test_crypto_data() {
    let crypto = Cypher::new("magickey");

    // test AES256
    let data = crypto
        .encrypt("http://magiclen.org", &Standard::AES256)
        .unwrap();
    assert_eq!("DS/2U8royDnJDiNY2ps3f6ZoTbpZo8ZtUGYLGEjwLDQ=", data);

    let data = crypto
        .decrypt(
            "DS/2U8royDnJDiNY2ps3f6ZoTbpZo8ZtUGYLGEjwLDQ=",
            &Standard::AES256,
        )
        .unwrap();
    assert_eq!("http://magiclen.org", data);

    // test DES64
    let data = crypto
        .encrypt("http://magiclen.org", &Standard::DES64)
        .unwrap();
    assert_eq!("nqIQCAbQ0TKs6x6eGRdwrouES803NhvC", data);

    let data = crypto
        .decrypt("nqIQCAbQ0TKs6x6eGRdwrouES803NhvC", &Standard::DES64)
        .unwrap();
    assert_eq!("http://magiclen.org", data);

    // test AES128
    let data = crypto
        .encrypt("http://magiclen.org", &Standard::AES128)
        .unwrap();
    assert_eq!("Pdpj9HqTAN7vY7Z9msMzlIXJcNQ5N+cIJsiQhLqyjVI=", data);

    let data = crypto
        .decrypt(
            "Pdpj9HqTAN7vY7Z9msMzlIXJcNQ5N+cIJsiQhLqyjVI=",
            &Standard::AES128,
        )
        .unwrap();
    assert_eq!("http://magiclen.org", data);

    // test AES192
    let data = crypto
        .encrypt("http://magiclen.org", &Standard::AES192)
        .unwrap();
    assert_eq!("p0X9IHMqaxA78T0X8Y9DnNeEmVXIgUxrXmeyUEO1Muo=", data);

    let data = crypto
        .decrypt(
            "p0X9IHMqaxA78T0X8Y9DnNeEmVXIgUxrXmeyUEO1Muo=",
            &Standard::AES192,
        )
        .unwrap();
    assert_eq!("http://magiclen.org", data);
}
