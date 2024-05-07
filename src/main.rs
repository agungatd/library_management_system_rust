use std::collections::HashMap;

// Book struct to store book information
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    borrowed: bool,
}

// Library struct to manage books
struct Library {
    books: HashMap<String, Book>, // Key: unique identifier (e.g., ISBN), Value: Book struct
}

impl Library {
    fn new() -> Self {
        Library { books: HashMap::new() }
    }

    // Add book with title and author, using references to avoid ownership transfer
    fn add_book(&mut self, title: &str, author: &str) {
        let book_id = format!("{}-{:x}", title, rand::random::<u64>()); // Generate unique identifier
        let new_book = Book {
            title: title.to_string(),
            author: author.to_string(),
            borrowed: false,
        };
        self.books.insert(book_id, new_book);
        println!("[SUCCESS] - New Book inserted!");
    }
    
    fn get_book_id(&self, title: &str, author: &str) -> Option<String> {
        if let Some(book_id) = self.books.iter()
                                                        .find(|(_, book)| book.title == title && book.author == author)
                                                        .map(|(key, _)| key.to_string()) {
            return Some(book_id); // Return the book ID
        }
        None
    }

    // List all books with title, author, and borrowed status
    fn list_books(&self) {
        println!("List of Books:");
        for (_, book) in self.books.iter() {
            println!(
                "{} by {} (Borrowed: {})",
                book.title, book.author, if book.borrowed { "Yes" } else { "No" }
            );
        }
    }

    // Borrow a book by modifying borrowed status using a mutable reference
    fn borrow_book(&mut self, title: &str, author: &str) -> Result<String, String> {
        let book_id = self.get_book_id(title, author);
        if let Some(id) = book_id {
            let book = self.books.get_mut(&id).ok_or_else(|| "Book not found!".to_string())?;
            if book.borrowed {
                return Err("Book already borrowed!".to_string());
            }
            book.borrowed = true;
        }
        Ok("[SUCCESS] - Book: {title} is borrowed!".to_string())
    }

    // Return a book by modifying borrowed status using a mutable reference
    fn return_book(&mut self, title: &str, author: &str) -> Result<String, String> {
        let book_id = self.get_book_id(title, author);
        if let Some(id) = book_id {
            let book = self.books.get_mut(&id).ok_or_else(|| "Book not found!".to_string())?;
            if !book.borrowed {
                println!("[ERROR] - Book not already borrowed!");
                return Err("Book not already borrowed!".to_string());
            }
            book.borrowed = false;
        }
        Ok("[SUCCESS] - Book: {title} is returned!".to_string())
    }
}

fn main() {
    let mut library = Library::new();
    library.add_book("The Rust Programming Language", "Steve Kogel");
    library.add_book("The Hitchhiker's Guide to the Galaxy", "Douglas Adams");

    library.list_books();

    library.borrow_book("The Rust Programming Language", "Steve Kogel").unwrap();
    library.list_books();

    library.return_book("The Hitchhiker's Guide to the Galaxy", "Douglas Adams").err().unwrap();
    library.list_books();
}

