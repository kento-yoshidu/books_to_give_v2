use serde::Deserialize;
use askama::Template;

#[derive(Debug, Deserialize)]
struct Book {
    isbn: String,
    title: String,
    cover: Option<String>,
    tags: Vec<String>,
    rating: u8,
    added: String,
    memo: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Library {
    books: Vec<Book>,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexPage {
    books: Vec<Book>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string("books.toml")?;

    let library = toml::from_str::<Library>(&text)?;

    let page = IndexPage {
        books: library.books,
    };

    let html = page.render()?;

    std::fs::create_dir_all("dist")?;
    std::fs::write("dist/index.html", html)?;
    std::fs::copy("static/style.css", "dist/style.css")?;

    Ok(())
}
