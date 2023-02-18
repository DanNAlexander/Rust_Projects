use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

table! {
    books (id) {
        id -> Integer,
        title -> Text,
        author -> Text,
        publication_year -> Integer,
    }
}

#[derive(Queryable)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub publication_year: i32,
}

#[derive(Insertable)]
#[table_name="books"]
pub struct NewBook<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub publication_year: &'a i32,
}

pub fn create_book<'a>(conn: &SqliteConnection, title: &'a str, author: &'a str, publication_year: &'a i32) -> Book {
    use diesel::insert_into;

    let new_book = NewBook {
        title,
        author,
        publication_year,
    };

    insert_into(books::table)
        .values(&new_book)
        .execute(conn)
        .expect("Error saving new book");

    books::table
        .order(books::id.desc())
        .first(conn)
        .unwrap()
}

pub fn all_books(conn: &SqliteConnection) -> Vec<Book> {
    books::table
        .order(books::id)
        .load::<Book>(conn)
        .unwrap()
}
