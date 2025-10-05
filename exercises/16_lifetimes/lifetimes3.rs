// Lifetimes are also needed when structs hold references.

// TODO: Fix the compiler errors about the struct.
// is it a good idea to give the lifetime tag a "name" that refelcts the structs or object?
struct Book<'book> {
    author: &'book str,
    title: &'book str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
