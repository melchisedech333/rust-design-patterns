
use std::fmt;
use std::cmp::Ordering;
use std::cmp::Ordering::*;

enum BookFormat { Paperback, Hardback, Ebook }

struct Book {
    isbn: i32,
    format: BookFormat,
}

// Implementa igualdade parcial.
impl Eq for Book {}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }
}

// Implementa Order.
impl Ord for Book {
    fn cmp(&self, other: &Self) -> Ordering {
        self.isbn.cmp(&other.isbn)
    }
}

impl PartialOrd for Book {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Implementa Display.
impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ISBN: {}, FORMAT: {}", self.isbn, self.format)
    }
}

impl fmt::Display for BookFormat{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            BookFormat::Paperback => String::from("Paperback"),
            BookFormat::Hardback  => String::from("Hardback"),
            BookFormat::Ebook     => String::from("Ebook"),
        };

        write!(f, "{}", printable)
    }
}

// Exemplo de utilização dos tipos.
fn main() {

    // Display.
    let book1 = Book { isbn: 1, format: BookFormat::Paperback };
    let book2 = Book { isbn: 2, format: BookFormat::Hardback };
    let book3 = Book { isbn: 1, format: BookFormat::Ebook };

    println!("book1 -> {}", book1);
    println!("book2 -> {}", book2);
    println!("book3 -> {}", book3);
    
    // Igualdade.
    if book1 == book2 {
        println!("{} == {}", book1, book2);
    } else {
        println!("{} != {}", book1, book2);
    }

    if book1 == book3 {
        println!("{} == {}", book1, book3);
    } else {
        println!("{} != {}", book1, book3);
    }

    // Order.
    println!("{}", cmp_check(book1.cmp(&book2)));
    println!("{}", cmp_check(book2.cmp(&book1)));
    println!("{}", cmp_check(book1.cmp(&book3)));
    println!("{}", cmp_check(book2.cmp(&book3)));

    if book1 == book3 {
        println!("book1 == book3");
    } else {
        println!("book1 != book3");
    }

    if book2 > book1 {
        println!("book2 > book1");
    } else {
        println!("! (book2 > book1)");
    }

    if book2 >= book3 {
        println!("book2 >= book3");
    } else {
        println!("! (book2 >= book3)");
    }
}

fn cmp_check(result: Ordering) -> String {
    return match result {
        Less    => String::from("Less"),
        Greater => String::from("Greater"),
        Equal   => String::from("Equal"),
    }
}


