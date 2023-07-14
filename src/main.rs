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

fn joins(conn: &mut SqliteConnection) -> Result<(), Box<dyn Error + Send + Sync>> {
    let page_with_book = pages::table
        .inner_join(books::table)
        .filter(books::title.eq("Momo"))
        .select((Page::as_select(), Book::as_select()))
        .load::<(Page, Book)>(conn)?;

    println!("Page-Book pairs: {page_with_book:?}");

    let book_without_pages = books::table
        .left_join(pages::table)
        .select((Book::as_select(), Option::<Page>::as_select()))
        .load::<(Book, Option<Page>)>(conn)?;

    println!("Book-Page pairs (including empty books): {book_without_pages:?}");
    Ok(())
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

fn m_to_n_relations(conn: &mut SqliteConnection) -> Result<(), Box<dyn Error + Send + Sync>> {
    let astrid_lindgren = authors::table
        .filter(authors::name.eq("Astrid Lindgren"))
        .select(Author::as_select())
        .get_result(conn)?;

    // get all of Astrid Lindgren's books
    let books = BookAuthor::belonging_to(&astrid_lindgren)
        .inner_join(books::table)
        .select(Book::as_select())
        .load(conn)?;
    println!("Asgrid Lindgren books: {books:?}");

    let collaboration = books::table
        .filter(books::title.eq("Pippi and Momo"))
        .select(Book::as_select())
        .get_result(conn)?;

    // get authors for the collaboration
    let authors = BookAuthor::belonging_to(&collaboration)
        .inner_join(authors::table)
        .select(Author::as_select())
        .load(conn)?;
    println!("Authors for \"Pipi and Momo\": {authors:?}");

    // get a list of authors with all their books
    let all_authors = authors::table.select(Author::as_select()).load(conn)?;

    let books = BookAuthor::belonging_to(&authors)
        .inner_join(books::table)
        .select((BookAuthor::as_select(), Book::as_select()))
        .load(conn)?;

    let books_per_author: Vec<(Author, Vec<Book>)> = books
        .grouped_by(&all_authors)
        .into_iter()
        .zip(authors)
        .map(|(b, author)| (author, b.into_iter().map(|(_, book)| book).collect()))
        .collect();

    println!("All authors including their books: {books_per_author:?}");

    Ok(())
}