use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::error::Error;

pub mod model;
pub mod schema;

use crate::model::*;
use crate::schema::*;
fn main() {
}


fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

fn one_to_n_relations(conn: &mut SqliteConnection) -> Result<(), Box<dyn Error + Send + Sync>> {
    let momo = books::table
        .filter(books::title.eq("Momo"))
        .select(Book::as_select())
        .get_result(conn)?;

    // get pages for the book "Momo"
    let pages = Page::belonging_to(&momo)
        .select(Page::as_select())
        .load(conn)?;

    println!("Pages for \"Momo\": \n {pages:?}\n");

    let all_books = books::table.select(Book::as_select()).load(conn)?;

    // get all pages for all books
    let pages = Page::belonging_to(&all_books)
        .select(Page::as_select())
        .load(conn)?;

    // group the pages per book
    let pages_per_book = pages
        .grouped_by(&all_books)
        .into_iter()
        .zip(all_books)
        .map(|(pages, book)| (book, pages))
        .collect::<Vec<(Book, Vec<Page>)>>();

    println!("Pages per book: \n {pages_per_book:?}\n");

    Ok(())
}