#![allow(unused)]

/*
  Before running the code, make sure you have a MongoDB server running locally on the default port (27017). You can adjust the MongoDB connection string accordingly if needed.
*/

use serde::{Deserialize, Serialize};
use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{doc, oid::ObjectId};
use tokio::main;

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    title: String,
    author: String,
    genre: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Movie {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    title: String,
    director: String,
    genre: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum CatalogItem {
    Book(Book),
    Movie(Movie),
}

#[tokio::main]
async fn main() {
    // Connect to the MongoDB server
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let database = client.database("catalog_db");
    
    // Insert sample data
    insert_sample_data(&database).await;

    // Query and print catalog items
    query_catalog(&database).await;
}

async fn insert_sample_data(database: &mongodb::Database) {
    // Insert sample book
    let book = Book {
        id: None,
        title: "The Rust Programming Language".to_string(),
        author: "Steve Klabnik and Carol Nichols".to_string(),
        genre: "Programming".to_string(),
    };

    database.collection::<Book>("books").insert_one(book.clone(), None).await.unwrap();

    // Insert sample movie
    let movie = Movie {
        id: None,
        title: "Inception".to_string(),
        director: "Christopher Nolan".to_string(),
        genre: "Sci-Fi".to_string(),
    };

    database.collection::<Movie>("movies").insert_one(movie.clone(), None).await.unwrap();
}

async fn query_catalog(database: &mongodb::Database) {
    // Query books
    let books = database.collection::<Book>("books").find(None, None).await.unwrap();
    println!("Books:");
    for book in books {
        match book {
            Ok(b) => println!("{:?}", CatalogItem::Book(b)),
            Err(e) => println!("Error: {}", e),
        }
    }

    // Query movies
    let movies = database.collection::<Movie>("movies").find(None, None).await.unwrap();
    println!("Movies:");
    for movie in movies {
        match movie {
            Ok(m) => println!("{:?}", CatalogItem::Movie(m)),
            Err(e) => println!("Error: {}", e),
        }
    }
}
