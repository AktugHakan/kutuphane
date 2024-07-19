use std::collections::{self, HashMap};

use crate::book::Book;

pub struct Library {
    books: HashMap<u8, Book>,
}

impl Default for Library {
    fn default() -> Self {
        Self::new()
    }
}

impl Library {
    pub fn new() -> Self {
        let books = HashMap::new();
        Library { books }
    }

    pub fn get_from_barcode(&self, barcode: &u8) -> Option<&Book> {
        self.books.get(barcode)
    }

    pub fn add_book(&mut self, book: Book) {
        self.books.insert(book.get_barcode(), book);
    }

    pub fn create_book(&mut self, title: &str, barcode: u8) {
        let new_book = Book::new(title, barcode);
        self.add_book(new_book);
    }

    pub fn print_library(&self) {
        for (_, book) in self {
            println!("{}", book)
        }
    }

    pub fn delete_book(&mut self, barcode: u8) -> bool {
        self.books.remove(&barcode).is_some()
    }

    pub fn lend(&mut self, barcode: &u8, borrower_name: &str) -> Result<(), String> {
        if let Some(book) = self.books.get_mut(barcode) {
            book.lend_to(borrower_name)?;
            Ok(())
        } else {
            Err(format!("{} barkodlu kitap bulunamadı.", barcode))
        }
    }
}

impl IntoIterator for Library {
    type Item = (u8, Book);

    type IntoIter = collections::hash_map::IntoIter<u8, Book>;

    fn into_iter(self) -> Self::IntoIter {
        self.books.into_iter()
    }
}

impl<'a> IntoIterator for &'a Library {
    type Item = (&'a u8, &'a Book);

    type IntoIter = collections::hash_map::Iter<'a, u8, Book>;

    fn into_iter(self) -> Self::IntoIter {
        self.books.iter()
    }
}
