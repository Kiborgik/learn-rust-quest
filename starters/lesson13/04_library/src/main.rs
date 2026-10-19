use l13_04_library::catalog;

fn main() {
    match catalog::find("Borrow") {
        Some(book) => println!("Found {} ({} pages)", book.title, book.pages),
        None => println!("No such book"),
    }
}
