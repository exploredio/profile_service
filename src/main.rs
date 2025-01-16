use actix_web::{web, App, HttpServer};
use aws_sdk_dynamodb::{Client};
use aws_config::BehaviorVersion;
use aws_config::meta::region::RegionProviderChain;
use crate::dynamodb::{create_profile, get_profile, update_profile, delete_profile, delete_all_profiles};

mod dynamodb;
mod models {
    pub mod profile_request;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let region_provider = RegionProviderChain::default_provider().or_else("eu-central-1");
    let config = aws_config::defaults(BehaviorVersion::latest()).region(region_provider).load().await;
    let dynamodb_local_config = aws_sdk_dynamodb::config::Builder::from(&config).build();

    let client = Client::from_conf(dynamodb_local_config);

    let list_resp = client.list_tables().send().await;
    match list_resp {
        Ok(resp) => {
            println!("Found {} tables", resp.table_names().len());
            for name in resp.table_names() {
                println!("  {}", name);
            }
        }
        Err(err) => eprintln!("Failed to list local dynamodb tables: {err:?}"),
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(create_profile)
            .service(get_profile)
            .service(update_profile)
            .service(delete_all_profiles)
            .service(delete_profile)
    })
    .bind("127.0.0.1:8082")?
    .run()
    .await
}