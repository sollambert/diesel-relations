// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Integer,
        title -> Text,
    }
}

diesel::table! {
    pages (id) {
        id -> Integer,
        page_number -> Integer,
        content -> Text,
        book_id -> Integer,
    }
}

diesel::joinable!(pages -> books (book_id));

diesel::allow_tables_to_appear_in_same_query!(
    books,
    pages,
);
