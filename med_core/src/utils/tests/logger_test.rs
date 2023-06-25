use crate::{app::core::App, models::params::Params};

#[tokio::test]
async fn test_logger() {
    let new_params = Params {
        conf_path: "../demo/conf/conf_csv.yaml".to_owned(),
        debug: true,
        ..Default::default()
    };

    let new_app = App::new(new_params.clone()).await.unwrap();
    assert!(new_app.params.debug);
}
