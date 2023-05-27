use crate::{app::core::App, models::params::Params};

#[tokio::test]
async fn test_logger() {
    let mut new_params = Params::default();
    new_params.conf_path = "../demo/conf/conf_csv.yaml".to_owned();
    new_params.debug = true;

    let new_app = App::new(new_params.clone()).await.unwrap();
    assert_eq!(new_app.params.debug, true);
}
