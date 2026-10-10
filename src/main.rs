use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Book {
    isbn: String,
    title: String,
    cover: String,
    tags: Vec<String>,
    rating: u8,
    added: String,
    memo: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Library {
    books: Vec<Book>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string("books.toml")?;

    let library = toml::from_str::<Library>(&text)?;

    let mut content = format!("<!DOCTYPE html>
<html lang=\"ja\">
  <head>
      <meta charset=\"utf-8\">
      <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
      <meta name=\"robots\" content=\"noindex, nofollow\">
  </head>
  <body>
    <div class=\"main\">
      <ul>");

    for b in library.books {
        content.push_str(&format!("<li>{}</li>\n", b.title));
    }

    content.push_str("
      </ul>
    </div>
  </body>
</html>");

    std::fs::create_dir_all("dist")?;
    std::fs::write("dist/index.html", &content)?;

    Ok(())
}
