struct Book {
    title: String,
    author: String,
    isbn: String,
    available: bool
}

impl Book {
    fn new(title: String, author: String, isbn: String) -> Self {
        Self {title, author, isbn, available: true}
    }

    fn change_availability(&mut self) {
        self.available = !self.available;
    }    
}

struct Library {
    id: u32,
    books: Vec<Book>,
}

struct Patron {
    id: u32,
    name: String,
    checkouts: Vec<String>,
}

impl Library {
    fn new(id: u32) -> Self{
        Self {id, books: vec![]}
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn remove_book(&mut self, book: &Book) {
        self.books.retain(|b| b.isbn != book.isbn);
    }
}

impl Patron {
    fn new(id: u32, name: String) -> Self {
        Self { id, name, checkouts: vec![] }
    }

    fn check_out_book(&mut self, book: &mut Book) {
        self.checkouts.push(book.isbn.clone());
        book.change_availability();
    }

    fn return_book(&mut self, book: &mut Book) {
        self.checkouts.retain(|b| String::from(b) == book.isbn);
        book.change_availability();
    }
}

struct LibrarySystem {
    libraries: Vec<Library>
}

impl<'a> LibrarySystem {
    fn new() -> Self {
        Self { libraries: vec![] }
    }

    fn add_library(&mut self, library: Library) {
        self.libraries.push(library);
    }

    fn get_available_books(&mut self) -> Vec<&mut Book> {
        let mut books: Vec<&mut Book> = vec![];
        for lib in &mut self.libraries {
            books.extend(lib.books.iter_mut().filter(|b| b.available));
        }

        books
    }
}

fn main() {
    let book1 = Book::new(String::from("Title 1"), String::from("Author 1"), String::from("ISBN 1"));
    let book2 = Book::new(String::from("Title 2"), String::from("Author 2"), String::from("ISBN 2"));
    let book3 = Book::new(String::from("Title 3"), String::from("Author 3"), String::from("ISBN 3"));

    let mut library1 = Library::new(1);
    let mut library2 = Library::new(2);

    library1.add_book(book1);
    library1.add_book(book2);
    library2.add_book(book3);

    let mut lib_system = LibrarySystem::new();
    lib_system.add_library(library1);
    lib_system.add_library(library2);

    let mut patron = Patron::new(1, String::from("Patron"));
    let mut available_books = lib_system.get_available_books();

    if let Some(book) = available_books.first_mut() {
        patron.check_out_book(book);
        patron.return_book(book);
    }

}
