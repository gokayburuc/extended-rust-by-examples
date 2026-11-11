// ============================================================
// KONU     : Kısmi Eşitlik (`PartialEq`)
// KAYNAK   : https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
// AÇIKLAMA : `PartialEq` trait'ini implemente ederek özel tiplerde
//            `==` ve `!=` operatörlerinin nasıl tanımlanacağını gösterir.
// ============================================================

#[derive(Debug)]
enum BookFormat {
    Paperback,
    Hardback,
    Ebook,
}

struct Book {
    isbn: i32,
    format: BookFormat,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }
}

pub fn partialeq_execute() {
    let b1 = Book {
        isbn: 3,
        format: BookFormat::Paperback,
    };

    let b2 = Book {
        isbn: 3,
        format: BookFormat::Ebook,
    };

    let b3 = Book {
        isbn: 10,
        format: BookFormat::Paperback,
    };

    let b4: Book = Book {
        isbn: 11,
        format: BookFormat::Hardback,
    };

    assert!(b1 == b2);
    assert!(b1 != b3);
    assert!(b1 != b4);
}
