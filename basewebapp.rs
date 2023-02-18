use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use rocket::http::RawStr;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::templates::Template;

mod database;

use database::{all_books, create_book, establish_connection};

#[derive(FromForm)]
struct BookForm {
    title: String,
    author: String,
    publication_year: i32,
}

#[get("/")]
fn index(conn: State<SqliteConnection>) -> Template {
    let books = all_books(&conn);
    let context = vec![("books", &books)];
    Template::render("index", &context)
}

#[post("/", data = "<book_form>")]
fn create(conn: State<SqliteConnection>, book_form: Form<BookForm>) -> Redirect {
    let BookForm {
        title,
        author,
        publication_year,
    } = book_form.into_inner();

    create_book(&conn, &title, &author, &publication_year);

    Redirect::to(uri!(index))
}

fn main() {
    rocket::ignite()
        .manage(establish_connection())
        .attach(Template::fairing())
        .mount("/", routes![index, create])
        .launch();
}
