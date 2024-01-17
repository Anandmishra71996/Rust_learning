// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use chrono::{DateTime, Utc};
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use mongodb::{bson, bson::Document,bson::doc, options::{ClientOptions, FindOptions}, Client};
use serde::{Deserialize, Serialize};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// impl Clone for Tasks {
//     fn clone(&self) -> Self {
//         Self {
//             task_list: self.task_list.clone(),
//             last_added_at: self.last_added_at.clone(), // Clone the String field
//         }
//     }
// }
#[derive(Debug, Serialize, Deserialize)]
struct Todo{
    task:String,
    added_at:String
}


// remember to call `.manage(MyState::default())`
#[tauri::command]
async fn create_todo(client: tauri::State<'_, Client>,todo: Todo) -> Result<(), String> {
    println!("{:?}",todo);
    let db = client.database("rust_tut".into());
    let target_collection=db.collection::<Todo>("todo");
    let new_todo=vec![todo];
   let data= target_collection.insert_many(&new_todo,None).await;
    println!("{:?}",data);

  Ok(())
}
#[tauri::command]
async fn get_todo(client: tauri::State<'_, Client>) -> Result<Vec<Document>, String> {
    let db = client.database("rust_tut".into());
    let target_collection=db.collection::<Document>("todo");
   let mut cursor= target_collection.find(None,FindOptions::default()).await.expect("Some error message");;
//    let r=Some(cursor?.deserialize_current());
//    println!("{:?}",r);
   let mut results = Vec::new();
//    results.push(Todo{
//     task:"task".into(),
//     added_at:"dd".into()
//    });
   while cursor.advance().await.expect("Some error message") {
       results.push(cursor.deserialize_current().expect("Some error message"));
   }
println!("{:?}",results);
   Ok(results)

}
#[tauri::command]
async fn connect( client: tauri::State<'_, Client>)->Result<String,()>{
    let db = client.database("rust_tut".into());
    let target_collection=db.collection::<Document>("todo".into());
   
    println!("connected");
   Ok("Connected".into())
    // let target_collection = db.collection::<Document>(&collection);
}

 fn main() {
    dotenv().ok();
    let client_uri =
    env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let options = ClientOptions::parse(client_uri).expect("invalid database url");

    let client = Client::with_options(options).unwrap();
    // let db_url = "mongodb://localhost:27017/demo";

    // let options = ClientOptions::parse(db_url).expect("invalid database url");

    // let client = Client::with_options(options).unwrap();
    tauri::Builder::default()
    .manage(client)
        .invoke_handler(tauri::generate_handler![greet,create_todo,get_todo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
