# Storing Book

Let's  model a library's book collection. Copy the code below to
<https://play.rust-lang.org/> and update the types to make it compile:

```rust,should_panic
{{#include book-library.rs:setup}}
{{#include book-library.rs:Library_new}}
        todo!("Initialize and return a `Library` value")
    }

{{#include book-library.rs:Library_len}}
        todo!("Return the length of `self.books`")
    }

{{#include book-library.rs:Library_is_empty}}
        todo!("Return `true` if `self.books` is empty")
    }

{{#include book-library.rs:Library_add_book}}
        todo!("Add a new book to `self.books`")
    }

{{#include book-library.rs:Library_print_books}}
        todo!("Iterate over `self.books` and print each book's title and year")
    }

{{#include book-library.rs:Library_oldest_book}}
        todo!("Return a reference to the oldest book (if any)")
    }
}

{{#include book-library.rs:main}}
```
