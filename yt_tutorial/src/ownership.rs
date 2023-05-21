struct Book {
    pages: i32,
    rating: i32,
}

fn display_page_count(book: &Book) {
    println!("pages : {:?}", book.pages);
}

pub(crate) fn ownership_demo() {
    let book = Book {
        pages: 5,
        rating: 9,
    };
    display_page_count(&book);
    display_page_count(&book);
}
