struct Book {
    pages: i32,
    rating: i32,
}

// the pass by refrence & sign is being placed at datatype not to actual data
fn display_page(book: &Book) {
    println!("the number of pages is: {:?}", book.pages);
}
fn display_rating(book: &Book) {
    println!("the number of pages is: {:?}", book.rating);
}

fn main() {
    let book = Book {
        pages: 344,
        rating: 4,
    };
    display_page(&book); // pass by reference working here
    display_rating(&book); // pass by reference working here
}
