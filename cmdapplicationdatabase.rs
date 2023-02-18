use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

mod database;

use database::{all_books, create_book, establish_connection};

fn main() {
    let conn = establish_connection();

    create_book(&conn, "The Great Gatsby", "F. Scott Fitzgerald", &1925);
    create_book(&conn, "To Kill a Mockingbird", "Harper Lee", &1960);

    let all_books = all_books(&conn);
    for book in all_books {
        println!("{} by {} ({})", book.title, book.author, book.publication_year);
    }
}
