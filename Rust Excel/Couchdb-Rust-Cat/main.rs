use serde::{Deserialize, Serialize};
use couchdb::{Client, Document, Server};
use tokio::main;

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
    genre: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Movie {
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
    // Connect to CouchDB server
    let client = Client::new("http://localhost:5984").unwrap();
    let server = Server::new(client);

    // Create a new database or use an existing one
    let database = server.db("catalog_db");

    // Insert sample data
    insert_sample_data(&database).await;

    // Query and print catalog items
    query_catalog(&database).await;
}

async fn insert_sample_data(database: &couchdb::Database) {
    // Insert sample book
    let book = Book {
        title: "The Rust Programming Language".to_string(),
        author: "Steve Klabnik and Carol Nichols".to_string(),
        genre: "Programming".to_string(),
    };

    database.create_document(Document::new(book.clone())).await.unwrap();

    // Insert sample movie
    let movie = Movie {
        title: "Inception".to_string(),
        director: "Christopher Nolan".to_string(),
        genre: "Sci-Fi".to_string(),
    };

    database.create_document(Document::new(movie.clone())).await.unwrap();
}

async fn query_catalog(database: &couchdb::Database) {
    // Query books
    let books: Vec<Book> = database
        .all_docs()
        .await
        .unwrap()
        .documents
        .into_iter()
        .filter_map(|doc| serde_json::from_value(doc.json).ok())
        .collect();
    
    println!("Books: {:?}", books);

    // Query movies
    let movies: Vec<Movie> = database
        .all_docs()
        .await
        .unwrap()
        .documents
        .into_iter()
        .filter_map(|doc| serde_json::from_value(doc.json).ok())
        .collect();
    
    println!("Movies: {:?}", movies);
}
