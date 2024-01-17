use std::sync::Mutex;
use once_cell::sync::Lazy;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
    error::Error,
};
use dotenv::dotenv;
use std::env;
// use mongodb::{Client, options::{ClientOptions, ResolverConfig}};

static CLIENT: Lazy<Mutex<Client>> = Lazy::new(async ||{
    dotenv().ok();
    let client_uri =
    env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let resolver_config = ResolverConfig::cloudflare();
    let client_options = ClientOptions::parse_with_resolver_config(&client_uri, resolver_config).await.unwrap();
    let client=Client::with_options(client_options);
    Mutex::new(client)
});

// async fn get_client() -> Result<Mutex<Client>, Error> {
//     CLIENT.get_or_try_init(|| async {
//         let client_uri =
//         env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
  
//         let resolver_config = ResolverConfig::cloudflare();
//         let client_options = ClientOptions::parse_with_resolver_config(&client_uri, resolver_config).await?;
//         let client=Client::with_options(client_options)
//         Ok(Mutex::new(Client::with_options(client_options)?))
//     })
// }

pub async fn get_db(database_name: &str) -> Result<mongodb::Database, Error> {
    // let client = get_client().await?;
    let client_new=CLIENT.lock().unwrap();
    Ok(client_new.database(database_name))
}