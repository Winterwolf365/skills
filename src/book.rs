use crate::*;

pub fn filer_books(args: Args, mut books: Vec<Book>) -> Vec<Book> {
    if let Some(id) = args.id {
        books = books
            .iter()
            .filter(|book| book.id == id)
            .cloned()
            .collect::<Vec<Book>>();
    }

    if let Some(author) = args.author {
        books = books
            .iter()
            .filter(|book| book.author.to_lowercase().contains(&author.to_lowercase()))
            .cloned()
            .collect::<Vec<Book>>();
    }

    if let Some(genre) = args.genre {
        books = books
            .iter()
            .filter(|book| book.genre.to_lowercase().contains(&genre.to_lowercase()))
            .cloned()
            .collect::<Vec<Book>>();
    }

    if let Some(title) = args.title {
        books = books
            .iter()
            .filter(|book| book.title.to_lowercase().contains(&title.to_lowercase()))
            .cloned()
            .collect::<Vec<Book>>();
    }

    if let Some(language) = args.language {
        books = books
            .iter()
            .filter(|book| {
                book.language
                    .to_lowercase()
                    .contains(&language.to_lowercase())
            })
            .cloned()
            .collect::<Vec<Book>>();
    }

    books
}

pub fn sort_books(sort: Sort, mut books: Vec<Book>) -> Vec<Book> {
    match sort {
        Sort::Id => {}
        Sort::PublicationDate => books.sort_by_key(|book| book.publication_date),
        Sort::Author => books.sort_by_key(|book| book.author),
        Sort::Genre => books.sort_by_key(|book| book.genre),
        Sort::Title => books.sort_by_key(|book| book.title),
        Sort::TotalPages => books.sort_by_key(|book| book.total_pages),
        // book.rating is an float but the sort_by_key takes an integer the
        // mutiplication is so i don't lose information after the comma.
        Sort::Rating => books.sort_by_key(|book| (book.rating * 100.0) as u32),
        Sort::Language => books.sort_by_key(|book| book.language),
    }

    books
}

pub fn format_books(format: Format, books: Vec<Book>) -> String {
    match format {
        Format::RustStruct => return format!("{:#?}", books),
        Format::Json => return serde_json::to_string_pretty(&books).unwrap(),
        Format::Yaml => return serde_yaml::to_string(&books).unwrap(),
    }
}
