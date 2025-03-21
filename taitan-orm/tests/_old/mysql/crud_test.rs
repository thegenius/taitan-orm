use sqlx::sqlx_macros;
use testcontainers::GenericImage;
use testcontainers_modules::mysql;
use testcontainers::core::IntoContainerPort;
use testcontainers::core::WaitFor;
use testcontainers::runners::AsyncRunner;

#[sqlx_macros::test]
async fn test_mysql_crud() -> anyhow::Result<()> {
    // let container_result = GenericImage::new("mysql", "8")
    //     .with_exposed_port(3306.tcp())
    //     .with_wait_for(WaitFor::seconds(2))
    //     .start().await;
    // match  container_result {
    //     Ok(_) => (),
    //     Err(error) => {
    //         println!("{:#?}", error);
    //         assert!(false, "{:#?}", error);
    //     }
    // }


    Ok(())
}